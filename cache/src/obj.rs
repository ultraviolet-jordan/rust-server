use std::collections::HashMap;
use std::time::Instant;

use io::{JagFile, Packet};

use crate::param::{decode_params, ParamValue};

pub struct ObjProvider {
    names: HashMap<String, usize>,
    objs: Vec<ObjType>,
}

impl ObjProvider {
    /// Loads obj data from the specified directory, reading and decoding the required
    /// `.dat` files for both client and server objs.
    ///
    /// This function initializes an `ObjProvider` by loading obj data and indexing
    /// information from the provided directory path. It reads two `.dat` files:
    /// one for the client configuration (`client/config`) and one for the server
    /// objs (`server/obj.dat`). The function decodes the contents and builds
    /// a collection of obj types and their associated names.
    ///
    /// # Arguments
    ///
    /// * `dir` - A string slice representing the directory path where the obj data
    ///           files are located. The function expects the `client/config` file and
    ///           `server/obj.dat` file to be present.
    ///
    /// # Returns
    ///
    /// Returns an `ObjProvider` containing a collection of decoded obj configurations
    /// (`configs`) and a mapping of obj names (`names`) to their respective obj IDs.
    ///
    /// # Panics
    ///
    /// The function will panic if either the `client/config` file or the `server/obj.dat` file
    /// cannot be read, or if they contain unexpected data that cannot be decoded.
    #[rustfmt::skip]
    pub fn io(dir: &str, members: bool) -> ObjProvider {
        let start: Instant = Instant::now();
        match JagFile::new(std::fs::read(format!("{}/client/config", dir)).unwrap()).read("obj.dat") {
            Some(mut client) => {
                let mut server: Packet = Packet::io(format!("{}/server/obj.dat", dir));

                let count: usize = server.g2() as usize;
                client.pos += 2;

                let mut names: HashMap<String, usize> = HashMap::new();
                let mut objs: Vec<ObjType> = Vec::with_capacity(count);

                for id in 0..count {
                    let mut obj: ObjType = ObjType::new(id);
                    obj.decode(&mut server);
                    obj.decode(&mut client);

                    if let Some(debugname) = &obj.debugname {
                        names.insert(debugname.clone(), id);
                    }
                    objs.push(obj);
                }

                for id in 0..count {
                    if let Some(certtemplate) = objs.get(id).and_then(|obj| obj.certtemplate) {
                        match objs.get(certtemplate as usize) {
                            None => panic!("Obj not found for a certtemplate!"),
                            Some(template) => {
                                let model: u16 = template.model;
                                let zoom2d: u16 = template.zoom2d;
                                let xan2d: u16 = template.xan2d;
                                let yan2d: u16 = template.yan2d;
                                let zan2d: u16 = template.zan2d;
                                let xof2d: i16 = template.xof2d;
                                let yof2d: i16 = template.yof2d;
                                let recol_s: Option<Vec<u16>> = template.recol_s.clone();
                                let recol_d: Option<Vec<u16>> = template.recol_d.clone();
                                if let Some(obj) = objs.get_mut(id) {
                                    obj.cert_template(model, zoom2d, xan2d, yan2d, zan2d, xof2d, yof2d, recol_s, recol_d);
                                }
                            }
                        }
                    }
                    if let Some(certlink) = objs.get(id).and_then(|obj| obj.certlink) {
                        match objs.get(certlink as usize) {
                            None => panic!("Obj not found for a certlink!"),
                            Some(link) => {
                                let name: Option<String> = link.name.clone();
                                let members: bool = link.members;
                                let cost: i32 = link.cost;
                                let tradeable: bool = link.tradeable;
                                if let Some(obj) = objs.get_mut(id) {
                                    obj.cert_link(name, members, cost, tradeable);
                                }
                            }
                        }
                    }
                    if let Some(obj) = objs.get_mut(id) {
                        obj.disable(members);
                    }
                }
                println!("Loaded objs in: {:?}", start.elapsed());
                return ObjProvider { names, objs };
            }
            _ => panic!("Could not load objs!"),
        }
    }

    /// Retrieves an obj by its ID, invoking the provided callback functions
    /// based on whether the obj is found or not.
    ///
    /// This method looks up the obj in the internal `configs` collection using
    /// the provided `id`. If the obj is found, the `on_found` callback is invoked
    /// with a reference to the obj. If the obj is not found, the `on_not_found`
    /// callback is invoked.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the obj to retrieve from the `configs`.
    /// * `on_found` - A closure that is called with a reference to the found `ObjType`
    ///                if the obj exists in the `configs` collection.
    /// * `on_not_found` - A closure that is called if the obj does not exist in
    ///                    the `configs` collection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cache::ObjProvider;
    ///
    /// let provider = ObjProvider::io("../data/pack", true);
    /// provider.get_by_id(1046, |obj| {
    ///     if let Some(name) = &obj.name {
    ///         println!("Found obj with name: {}", name);
    ///     } else {
    ///         println!("Found obj with no name.");
    ///     }
    /// }, || {
    ///     println!("obj not found!");
    /// });
    /// ```
    ///
    /// # Behavior
    ///
    /// - If the obj with the given `id` is found in `configs`, the `on_found` closure
    ///   will be executed with the found `ObjType`.
    /// - If the obj is not found, the `on_not_found` closure will be executed.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    /// However, the callbacks may panic if they are implemented to do so.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data structures of `ObjProvider` are properly initialized.
    ///
    /// # Side Effects
    ///
    /// - Calls the appropriate callback (`on_found` or `on_not_found`) based on the existence of the obj with the given name.
    ///
    /// # Performance
    ///
    /// The time complexity of this function is O(1), as it performs a direct lookup
    /// on the `configs` vector using the index.
    pub fn get_by_id<F, E>(&self, id: usize, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ObjType),
        E: FnOnce(),
    {
        if let Some(obj) = self.objs.get(id) {
            on_found(obj);
        } else {
            on_not_found();
        }
    }

    /// Retrieves an obj by its name and executes a callback based on whether the obj is found or not.
    ///
    /// This method searches for the obj name in the internal `names` map. If found, it retrieves the obj
    /// from the `configs` list and invokes the `on_found` callback. If the obj is not found, it invokes the
    /// `on_not_found` callback.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the obj to search for. This is a string slice (`&str`) and is used to look up the obj ID in the internal `names` map.
    /// - `on_found`: A callback function that takes a reference to the `ObjType` and is executed when the obj is found.
    /// - `on_not_found`: A callback function that takes no arguments and is executed when the obj is not found.
    ///
    /// # Return
    ///
    /// This function does not return any value. Instead, it calls the provided callbacks based on whether the obj is found or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cache::ObjProvider;
    ///
    /// let provider = ObjProvider::io("../data/pack", false);
    /// provider.get_by_name("christmas_cracker", |obj| {
    ///     if let Some(name) = &obj.name {
    ///         println!("Found obj with name: {}", name);
    ///     } else {
    ///         println!("Found obj with no name.");
    ///     }
    /// }, || {
    ///     println!("obj not found!");
    /// });
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    /// However, the callbacks may panic if they are implemented to do so.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal data structures of `ObjProvider` are properly initialized.
    ///
    /// # Side Effects
    ///
    /// - Calls the appropriate callback (`on_found` or `on_not_found`) based on the existence of the obj with the given name.
    ///
    /// # Performance
    ///
    /// This function performs two lookups: one in the `names` map and another in the `configs` list. If the name does not exist,
    /// it performs minimal work. The time complexity is O(1) for both lookups.
    pub fn get_by_name<F, E>(&self, name: &str, on_found: F, on_not_found: E)
    where
        F: FnOnce(&ObjType),
        E: FnOnce(),
    {
        if let Some(id) = self.names.get(name) {
            self.get_by_id(*id, on_found, on_not_found);
        } else {
            on_not_found();
        }
    }
}

pub struct ObjType {
    pub id: usize,  // 0->65535
    pub model: u16, // 0->65535
    pub name: Option<String>,
    pub desc: Option<String>,
    pub zoom2d: u16, // 0->65535
    pub xan2d: u16,  // 0->65535
    pub yan2d: u16,  // 0->65535
    pub xof2d: i16,  // -1->32767
    pub yof2d: i16,  // -1->32767
    pub code9: bool,
    pub code10: Option<u16>, // -1->65535
    pub stackable: bool,
    pub cost: i32,            // -2147483648->2147483647
    pub wearpos: Option<u8>,  // -1->255
    pub wearpos2: Option<u8>, // -1->255
    pub members: bool,
    pub manwear: Option<u16>,    // -1->65535
    pub manwear2: Option<u16>,   // -1->65535
    pub manweary: i8,            // -128->127
    pub womanwear: Option<u16>,  // -1->65535
    pub womanwear2: Option<u16>, // -1->65535
    pub womanweary: i8,          // -128->127
    pub wearpos3: Option<u8>,    // -1->255
    pub op: Option<Vec<Option<String>>>,
    pub iop: Option<Vec<Option<String>>>,
    pub recol_s: Option<Vec<u16>>,
    pub recol_d: Option<Vec<u16>>,
    pub weight: i16,                // -32768->32767
    pub manwear3: Option<u16>,      // -1->65535
    pub womanwear3: Option<u16>,    // -1->65535
    pub manhead: Option<u16>,       // -1->65535
    pub manhead2: Option<u16>,      // -1->65535
    pub womanhead: Option<u16>,     // -1->65535
    pub womanhead2: Option<u16>,    // -1->65535
    pub category: Option<u16>,      // -1->65535
    pub zan2d: u16,                 // 0->65535
    pub dummyitem: u8,              // 0->255
    pub certlink: Option<u16>,      // -1->65535
    pub certtemplate: Option<u16>,  // -1->65535
    pub countobj: Option<Vec<u16>>, // -1->65535
    pub countco: Option<Vec<u16>>,  // -1->65535
    pub tradeable: bool,
    pub respawnrate: u16, // 0->65535
    pub params: Option<HashMap<i32, ParamValue>>,
    pub debugname: Option<String>,
}

impl ObjType {
    fn new(id: usize) -> ObjType {
        return ObjType {
            id,
            model: 0,
            name: None,
            desc: None,
            zoom2d: 2000,
            xan2d: 0,
            yan2d: 0,
            xof2d: 0,
            yof2d: 0,
            code9: false,
            code10: None,
            stackable: false,
            cost: 1,
            wearpos: None,
            wearpos2: None,
            members: false,
            manwear: None,
            manwear2: None,
            manweary: 0,
            womanwear: None,
            womanwear2: None,
            womanweary: 0,
            wearpos3: None,
            op: None,
            iop: None,
            recol_s: None,
            recol_d: None,
            weight: 0,
            manwear3: None,
            womanwear3: None,
            manhead: None,
            manhead2: None,
            womanhead: None,
            womanhead2: None,
            category: None,
            zan2d: 0,
            dummyitem: 0,
            certlink: None,
            certtemplate: None,
            countobj: None,
            countco: None,
            tradeable: false,
            respawnrate: 100,
            params: None,
            debugname: None,
        };
    }

    #[rustfmt::skip]
    fn decode(&mut self, dat: &mut Packet) {
        while dat.remaining() > 0 {
            let code: u8 = dat.g1();
            match code {
                0 => break,
                1 => self.model = dat.g2(),
                2 => self.name = Some(dat.gjstr(10)),
                3 => self.desc = Some(dat.gjstr(10)),
                4 => self.zoom2d = dat.g2(),
                5 => self.xan2d = dat.g2(),
                6 => self.yan2d = dat.g2(),
                7 => {
                    let mut xof2d: i32 = dat.g2() as i32;
                    if xof2d > 32767 {
                        xof2d -= 65536;
                    }
                    self.xof2d = xof2d as i16;
                }
                8 => {
                    let mut yof2d: i32 = dat.g2() as i32;
                    if yof2d > 32767 {
                        yof2d -= 65536;
                    }
                    self.xof2d = yof2d as i16;
                }
                9 => self.code9 = true, // animHasAlpha from code10?
                10 => self.code10 = Some(dat.g2()), // seq?
                11 => self.stackable = true,
                12 => self.cost = dat.g4s(),
                13 => self.wearpos = Some(dat.g1()),
                14 => self.wearpos2 = Some(dat.g1()),
                16 => self.members = true,
                23 => {
                    self.manwear = Some(dat.g2());
                    self.manweary = dat.g1s();
                }
                24 => self.manwear2 = Some(dat.g2()),
                25 => {
                    self.womanwear = Some(dat.g2());
                    self.womanweary = dat.g1s();
                }
                26 => self.womanwear2 = Some(dat.g2()),
                27 => self.wearpos3 = Some(dat.g1()),
                30..=34 => self.op.get_or_insert_with(|| vec![None; 5])[code as usize - 30] = Some(dat.gjstr(10)),
                35..=39 => self.iop.get_or_insert_with(|| vec![None; 5])[code as usize - 35] = Some(dat.gjstr(10)),
                40 => {
                    let count: usize = dat.g1() as usize;
                    let mut recol_s: Vec<u16> = vec![0; count];
                    let mut recol_d: Vec<u16> = vec![0; count];
                    for index in 0..count {
                        recol_s[index] = dat.g2();
                        recol_d[index] = dat.g2();
                    }
                    self.recol_s = Some(recol_s);
                    self.recol_d = Some(recol_d);
                }
                75 => self.weight = dat.g2s(),
                78 => self.manwear3 = Some(dat.g2()),
                79 => self.womanwear3 = Some(dat.g2()),
                90 => self.manhead = Some(dat.g2()),
                91 => self.womanhead = Some(dat.g2()),
                92 => self.manhead2 = Some(dat.g2()),
                93 => self.womanhead2 = Some(dat.g2()),
                94 => self.category = Some(dat.g2()),
                95 => self.zan2d = dat.g2(),
                96 => self.dummyitem = dat.g1(),
                97 => self.certlink = Some(dat.g2()),
                98 => self.certtemplate = Some(dat.g2()),
                100..=109 => {
                    self.countobj.get_or_insert_with(|| vec![0; 10])[code as usize - 100] = dat.g2();
                    self.countco.get_or_insert_with(|| vec![0; 10])[code as usize - 100] = dat.g2();
                }
                200 => self.tradeable = true,
                201 => self.respawnrate = dat.g2(),
                249 => decode_params(dat, self.params.get_or_insert_with(|| HashMap::new())),
                250 => self.debugname = Some(dat.gjstr(10)),
                _ => panic!("Error unrecognised obj config code: {}", code),
            }
        }
    }

    /// Configures the obj with the given certificate template parameters.
    ///
    /// This method sets various properties of the obj based on the provided certificate
    /// template values, such as model and 2D coordinates. It also updates the recolor settings
    /// if provided.
    ///
    /// # Parameters
    ///
    /// - `model`: A `u16` representing the model ID of the certificate.
    /// - `zoom2d`: A `u16` value for the zoom level in 2D space.
    /// - `xan2d`: A `u16` value representing the X-axis rotation in 2D space.
    /// - `yan2d`: A `u16` value representing the Y-axis rotation in 2D space.
    /// - `zan2d`: A `u16` value representing the Z-axis rotation in 2D space.
    /// - `xof2d`: An `i16` value representing the X-offset in 2D space.
    /// - `yof2d`: An `i16` value representing the Y-offset in 2D space.
    /// - `recol_s`: An `Option<Vec<u16>>` that specifies the source recoloring values.
    /// - `recol_d`: An `Option<Vec<u16>>` that specifies the destination recoloring values.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the obj in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the parameters
    /// provided are valid.
    ///
    /// # Side Effects
    ///
    /// - Updates the properties of the obj to reflect the certificate template configuration.
    fn cert_template(
        &mut self,
        model: u16,
        zoom2d: u16,
        xan2d: u16,
        yan2d: u16,
        zan2d: u16,
        xof2d: i16,
        yof2d: i16,
        recol_s: Option<Vec<u16>>,
        recol_d: Option<Vec<u16>>,
    ) {
        self.model = model;
        self.zoom2d = zoom2d;
        self.xan2d = xan2d;
        self.yan2d = yan2d;
        self.zan2d = zan2d;
        self.xof2d = xof2d;
        self.yof2d = yof2d;
        self.recol_s = recol_s;
        self.recol_d = recol_d;
    }

    /// Configures the obj based on a certificate link.
    ///
    /// This method updates various properties of the obj, including its name, membership status,
    /// cost, and tradeability. It also sets the stackable property and generates a description based
    /// on the name provided.
    ///
    /// # Parameters
    ///
    /// - `name`: An `Option<String>` representing the name of the obj. If `None`, the name will
    ///   not be set.
    /// - `members`: A `bool` indicating whether the obj is for members only.
    /// - `cost`: An `i32` representing the cost associated with the obj.
    /// - `tradeable`: A `bool` indicating whether the obj is tradeable.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the obj in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the internal
    /// state of the obj is valid.
    ///
    /// # Side Effects
    ///
    /// - Updates the properties of the obj and generates a description if a name is provided.
    fn cert_link(&mut self, name: Option<String>, members: bool, cost: i32, tradeable: bool) {
        self.name = name;
        self.members = members;
        self.cost = cost;
        self.tradeable = tradeable;
        self.stackable = true;
        if let Some(name) = &self.name {
            if let Some(char) = name.chars().next() {
                let article: &str = if "AEIOU".contains(char) { "an" } else { "a" };
                self.desc = Some(format!(
                    "Swap this note at any bank for {} {}.",
                    article, name
                ));
            }
        }
    }

    /// Disables the obj if it is a members-only obj on a non-members server.
    ///
    /// This method checks the membership status of the obj and, if the server does not support
    /// members and the obj is designated for members, it disables tradeability and clears any
    /// op properties associated with the obj.
    ///
    /// # Parameters
    ///
    /// - `members`: A `bool` indicating whether the server supports members.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the obj in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the obj’s
    /// properties are in a valid state.
    ///
    /// # Side Effects
    ///
    /// - Modifies the obj's tradeability and op properties based on the membership status.
    fn disable(&mut self, members: bool) {
        // If the obj is a members obj but the server is non-members
        if !members && self.members {
            self.tradeable = false;
            self.op = None;
            self.iop = None;

            // TODO: autodisable params here
        }
    }
}

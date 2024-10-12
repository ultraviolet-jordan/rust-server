use std::collections::HashMap;
use std::time::Instant;

use io::{JagFile, Packet};

use crate::param::{decode_params, ParamValue};

pub struct ObjProvider {
    names: HashMap<String, usize>,
    configs: Vec<ObjType>,
    members: bool,
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
                let mut configs: Vec<ObjType> = Vec::with_capacity(count);

                for id in 0..count {
                    let mut obj: ObjType = ObjType::new(id);
                    obj.decode(&mut server);
                    obj.decode(&mut client);

                    if let Some(debugname) = &obj.debugname {
                        names.insert(debugname.clone(), id);
                    }
                    configs.push(obj);
                }

                for obj in configs.iter_mut().take(count) {
                    // If certtemplate is present, convert to certificate
                    if obj.certtemplate.is_some() {
                        obj.to_certificate();
                    }

                    // If the obj is a members object but the server is non-members
                    if !members && obj.members {
                        obj.tradeable = false;
                        obj.op = None;
                        obj.iop = None;

                        // TODO: autodisable params here
                    }
                }

                println!("Loaded objs in: {:?}", start.elapsed());
                return ObjProvider { names, configs, members };
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
        if let Some(obj) = self.configs.get(id) {
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
    fn decode(&mut self, buf: &mut Packet) {
        while buf.remaining() > 0 {
            let code: u8 = buf.g1();
            match code {
                0 => break,
                1 => self.model = buf.g2(),
                2 => self.name = Some(buf.gjstr(10)),
                3 => self.desc = Some(buf.gjstr(10)),
                4 => self.zoom2d = buf.g2(),
                5 => self.xan2d = buf.g2(),
                6 => self.yan2d = buf.g2(),
                7 => {
                    let mut xof2d: i32 = buf.g2() as i32;
                    if xof2d > 32767 {
                        xof2d -= 65536;
                    }
                    self.xof2d = xof2d as i16;
                }
                8 => {
                    let mut yof2d: i32 = buf.g2() as i32;
                    if yof2d > 32767 {
                        yof2d -= 65536;
                    }
                    self.xof2d = yof2d as i16;
                }
                9 => self.code9 = true, // animHasAlpha from code10?
                10 => self.code10 = Some(buf.g2()), // seq?
                11 => self.stackable = true,
                12 => self.cost = buf.g4s(),
                13 => self.wearpos = Some(buf.g1()),
                14 => self.wearpos2 = Some(buf.g1()),
                16 => self.members = true,
                23 => {
                    self.manwear = Some(buf.g2());
                    self.manweary = buf.g1s();
                }
                24 => self.manwear2 = Some(buf.g2()),
                25 => {
                    self.womanwear = Some(buf.g2());
                    self.womanweary = buf.g1s();
                }
                26 => self.womanwear2 = Some(buf.g2()),
                27 => self.wearpos3 = Some(buf.g1()),
                30..=34 => self.op.get_or_insert_with(|| vec![None; 5])[code as usize - 30] = Some(buf.gjstr(10)),
                35..=39 => self.iop.get_or_insert_with(|| vec![None; 5])[code as usize - 35] = Some(buf.gjstr(10)),
                40 => {
                    let count: usize = buf.g1() as usize;
                    let mut recol_s: Vec<u16> = vec![0; count];
                    let mut recol_d: Vec<u16> = vec![0; count];
                    for index in 0..count {
                        recol_s[index] = buf.g2();
                        recol_d[index] = buf.g2();
                    }
                    self.recol_s = Some(recol_s);
                    self.recol_d = Some(recol_d);
                }
                75 => self.weight = buf.g2s(),
                78 => self.manwear3 = Some(buf.g2()),
                79 => self.womanwear3 = Some(buf.g2()),
                90 => self.manhead = Some(buf.g2()),
                91 => self.womanhead = Some(buf.g2()),
                92 => self.manhead2 = Some(buf.g2()),
                93 => self.womanhead2 = Some(buf.g2()),
                94 => self.category = Some(buf.g2()),
                95 => self.zan2d = buf.g2(),
                96 => self.dummyitem = buf.g1(),
                97 => self.certlink = Some(buf.g2()),
                98 => self.certtemplate = Some(buf.g2()),
                100..=109 => {
                    self.countobj.get_or_insert_with(|| vec![0; 10])[code as usize - 100] = buf.g2();
                    self.countco.get_or_insert_with(|| vec![0; 10])[code as usize - 100] = buf.g2();
                }
                200 => self.tradeable = true,
                201 => self.respawnrate = buf.g2(),
                249 => decode_params(buf, self.params.get_or_insert_with(|| HashMap::new())),
                250 => self.debugname = Some(buf.gjstr(10)),
                _ => panic!("Error unrecognised obj config code: {}", code),
            }
        }
    }

    fn to_certificate(&self) {}
}

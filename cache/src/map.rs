use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use io::Packet;

pub struct MapProvider {
    pub mapsquares: HashMap<u16, MapSquare>,
}

impl MapProvider {
    pub fn io(dir: &str) -> MapProvider {
        let start: Instant = Instant::now();
        let path = format!("{}/server/maps/", dir);

        let maps: Vec<String> = fs::read_dir(&path)
            .unwrap()
            .filter_map(|f| f.ok())
            .map(|f| f.file_name().to_string_lossy().to_ascii_lowercase())
            .filter(|s| s.starts_with("m"))
            .collect();

        let mut mapsquares: HashMap<u16, MapSquare> = HashMap::new();

        for m in maps {
            let split: Vec<&str> = m[1..].split("_").collect();

            if let (Some(x), Some(z)) = (split.get(0), split.get(1)) {
                let x: u16 = u16::from_str(x).unwrap();
                let z: u16 = u16::from_str(z).unwrap();
                let mx: u16 = x << 6;
                let mz: u16 = z << 6;

                let npcs: Vec<MapSquareNpc> =
                    MapProvider::load_npcs(Packet::io(format!("{}n{}_{}", &path, x, z)), mx, mz);

                let objs: Vec<MapSquareObj> =
                    MapProvider::load_objs(Packet::io(format!("{}o{}_{}", &path, x, z)), mx, mz);

                let lands: Vec<Option<MapSquareLand>> =
                    MapProvider::load_ground(Packet::io(format!("{}m{}_{}", &path, x, z)), mx, mz);

                let locs: Vec<MapSquareLoc> = MapProvider::load_locations(
                    Packet::io(format!("{}l{}_{}", &path, x, z)),
                    &lands,
                    mx,
                    mz,
                );

                let mapsquare = MapSquare {
                    id: x << 8 | z,
                    npcs,
                    objs,
                    locs,
                    lands,
                };

                mapsquares.insert(mapsquare.id, mapsquare);
            }
        }
        println!("Loaded game map in: {:?}", start.elapsed());
        return MapProvider { mapsquares };
    }

    pub fn mock() -> MapProvider {
        return MapProvider {
            mapsquares: HashMap::new(),
        };
    }

    fn load_npcs(mut buf: Packet, mx: u16, mz: u16) -> Vec<MapSquareNpc> {
        let mut npcs: Vec<MapSquareNpc> = Vec::new();
        while buf.remaining() > 0 {
            let (x, z, y) = MapSquare::unpack_coord(buf.g2());
            let coord_x: u16 = mx + x as u16;
            let coord_z: u16 = mz + z as u16;
            let count: u8 = buf.g1();
            for _ in 0..count {
                let id: u16 = buf.g2();
                npcs.push(MapSquareNpc {
                    y,
                    x: coord_x,
                    z: coord_z,
                    id,
                });
            }
        }
        return npcs;
    }

    fn load_objs(mut buf: Packet, mx: u16, mz: u16) -> Vec<MapSquareObj> {
        let mut objs: Vec<MapSquareObj> = Vec::new();
        while buf.remaining() > 0 {
            let (x, z, y) = MapSquare::unpack_coord(buf.g2());
            let coord_x: u16 = mx + x as u16;
            let coord_z: u16 = mz + z as u16;
            let count: u8 = buf.g1();
            for _ in 0..count {
                let id: u16 = buf.g2();
                let count: u8 = buf.g1();
                objs.push(MapSquareObj {
                    y,
                    x: coord_x,
                    z: coord_z,
                    id,
                    count,
                });
            }
        }
        return objs;
    }

    fn load_ground(mut buf: Packet, mx: u16, mz: u16) -> Vec<Option<MapSquareLand>> {
        let mut lands: Vec<Option<MapSquareLand>> = vec![None; 64 * 64 * 4];
        for y in 0..MapSquare::Y {
            for x in 0..MapSquare::X {
                let coord_x: u16 = mx + x as u16;
                for z in 0..MapSquare::Z {
                    let coord_z: u16 = mz + z as u16;
                    let coord: usize = MapSquare::pack_coord(x, z, y);
                    while let code = buf.g1() {
                        match code {
                            0 => break,
                            1 => {
                                buf.pos += 1;
                                break;
                            }
                            2..=49 => {
                                buf.pos += 1;
                            }
                            50..=81 => {
                                lands[coord] = Some(MapSquareLand {
                                    y: y as u8,
                                    x: coord_x,
                                    z: coord_z,
                                    flag: code - 49,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        return lands;
    }

    fn load_locations(
        mut buf: Packet,
        lands: &Vec<Option<MapSquareLand>>,
        mx: u16,
        mz: u16,
    ) -> Vec<MapSquareLoc> {
        let mut locs: Vec<MapSquareLoc> = Vec::new();
        let mut loc_id: i32 = -1;
        let mut loc_id_offset: i32 = buf.gsmart();
        while loc_id_offset != 0 {
            loc_id += loc_id_offset;

            let mut coord: i32 = 0;
            let mut coord_offset: i32 = buf.gsmart();

            while coord_offset != 0 {
                coord += coord_offset - 1;
                let (x, z, y) = MapSquare::unpack_coord(coord as u16);

                let info: u8 = buf.g1();
                coord_offset = buf.gsmart();

                let bridged: bool = if y == 1 {
                    match &lands[coord as usize] {
                        None => continue,
                        Some(land) => land.flag & MapSquareLand::BRIDGE,
                    }
                } else {
                    match &lands[MapSquare::pack_coord(x as usize, z as usize, 1)] {
                        None => continue,
                        Some(land) => land.flag & MapSquareLand::BRIDGE,
                    }
                } == MapSquareLand::BRIDGE;

                let level: i8 = if bridged { y as i8 - 1 } else { y as i8 };
                if level < 0 {
                    continue;
                }

                let coord_x: u16 = mx + x as u16;
                let coord_z: u16 = mz + z as u16;

                locs.push(MapSquareLoc {
                    y,
                    x: coord_x,
                    z: coord_z,
                    id: loc_id as u16,
                    shape: info >> 2,
                    angle: info & 0x3,
                })
            }
            loc_id_offset = buf.gsmart();
        }
        return locs;
    }
}

pub struct MapSquareNpc {
    pub y: u8,
    pub x: u16,
    pub z: u16,
    pub id: u16,
}

pub struct MapSquareObj {
    pub y: u8,
    pub x: u16,
    pub z: u16,
    pub id: u16,
    pub count: u8,
}

#[derive(Clone)]
pub struct MapSquareLand {
    pub y: u8,
    pub x: u16,
    pub z: u16,
    pub flag: u8,
}

impl MapSquareLand {
    pub const OPEN: u8 = 0x0;
    pub const BLOCKED: u8 = 0x1;
    pub const BRIDGE: u8 = 0x2;
    pub const ROOF: u8 = 0x4;
    pub const WALL: u8 = 0x8;
    pub const LOWMEMORY: u8 = 0x10;
}

pub struct MapSquareLoc {
    pub y: u8,
    pub x: u16,
    pub z: u16,
    pub id: u16,
    pub shape: u8,
    pub angle: u8,
}

pub struct MapSquare {
    pub id: u16,
    pub npcs: Vec<MapSquareNpc>,
    pub objs: Vec<MapSquareObj>,
    pub locs: Vec<MapSquareLoc>,
    pub lands: Vec<Option<MapSquareLand>>,
}

impl MapSquare {
    pub const X: usize = 64;
    pub const Y: usize = 4;
    pub const Z: usize = 64;

    const SIZE: usize = MapSquare::X * MapSquare::Y * MapSquare::Z;

    pub fn unpack_coord(packed: u16) -> (u8, u8, u8) {
        let z: u8 = (packed & 0x3f) as u8;
        let x: u8 = ((packed >> 6) & 0x3f) as u8;
        let y: u8 = ((packed >> 12) & 0x3) as u8;
        return (x, z, y);
    }

    pub fn pack_coord(x: usize, z: usize, y: usize) -> usize {
        return (z & 0x3f) | ((x & 0x3f) << 6) | ((y & 0x3) << 12);
    }
}

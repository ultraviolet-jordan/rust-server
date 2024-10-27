use crate::zone::zonemap::ZoneMap;
use cache::{MapProvider, MapSquare, MapSquareLand};
use std::cell::RefCell;

pub struct GameMap {
    pub zone_map: RefCell<ZoneMap>,
}

impl GameMap {
    pub fn new() -> GameMap {
        return GameMap {
            zone_map: RefCell::new(ZoneMap::new()),
        };
    }

    pub fn mock() -> GameMap {
        return GameMap {
            zone_map: RefCell::new(ZoneMap::new()),
        };
    }

    pub fn load_map(&mut self, mut map: MapProvider) {
        for mapsquare in map.mapsquares.values_mut() {
            while let Some(npc) = mapsquare.npcs.pop() {
                // TODO add static npc/members
            }
            while let Some(obj) = mapsquare.objs.pop() {
                // TODO add static obj/members
            }
            for y in 0..MapSquare::Y {
                for x in 0..MapSquare::X {
                    for z in 0..MapSquare::Z {
                        if let Some(land) = &mapsquare.lands[MapSquare::pack_coord(x, z, y)] {
                            unsafe {
                                if x % 7 == 0 && z % 7 == 0 {
                                    rsmod::allocateIfAbsent(
                                        land.x as i32,
                                        land.z as i32,
                                        land.y as i32,
                                    );
                                }

                                if (land.flag & MapSquareLand::ROOF) != MapSquareLand::OPEN {
                                    rsmod::changeRoof(
                                        land.x as i32,
                                        land.z as i32,
                                        land.y as i32,
                                        true,
                                    );
                                }

                                if (land.flag & MapSquareLand::BLOCKED) != MapSquareLand::BLOCKED {
                                    continue;
                                }

                                let bridged: bool = if y == 1 {
                                    land.flag & MapSquareLand::BRIDGE
                                } else {
                                    match &mapsquare.lands[MapSquare::pack_coord(x, z, 1)] {
                                        None => continue,
                                        Some(land) => land.flag & MapSquareLand::BRIDGE,
                                    }
                                } == MapSquareLand::BRIDGE;

                                let level: i8 = if bridged { y as i8 - 1 } else { y as i8 };
                                if level < 0 {
                                    continue;
                                }

                                rsmod::changeFloor(
                                    land.x as i32,
                                    land.z as i32,
                                    land.y as i32,
                                    true,
                                );
                            }
                        }
                    }
                }
            }
            while let Some(loc) = mapsquare.locs.pop() {
                // TODO add static loc/members/collision
            }

            // redundant discard?
            mapsquare.objs.clear();
            mapsquare.lands.clear();
            mapsquare.npcs.clear();
            mapsquare.locs.clear();
        }
        // discard
        map.mapsquares.clear();
    }
}

use crate::coord_grid::CoordGrid;
use crate::entity::lifetime::EntityLifetime;
use crate::entity::loc::Loc;
use crate::entity::obj::Obj;
use crate::zone::zone_map::ZoneMap;
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

    pub fn load_map(&mut self, map: &MapProvider) {
        for mapsquare in map.mapsquares.values() {
            for npc in &mapsquare.npcs {
                // TODO add static npc/members
            }

            for obj in &mapsquare.objs {
                // TODO members
                self.zone_map
                    .borrow_mut()
                    .zone(obj.x, obj.y, obj.z)
                    .add_static_obj(Obj::new(
                        CoordGrid::from(obj.x, obj.y, obj.z),
                        EntityLifetime::Respawn,
                        obj.id,
                        obj.count as u32,
                    ));
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

            for loc in &mapsquare.locs {
                // TODO members/collision
                self.zone_map
                    .borrow_mut()
                    .zone(loc.x, loc.y, loc.z)
                    .add_static_loc(Loc::new(
                        CoordGrid::from(loc.x, loc.y, loc.z),
                        1, // TODO
                        1, // TODO
                        EntityLifetime::Respawn,
                        loc.id,
                        loc.shape,
                        loc.angle,
                    ));
            }
        }
    }
}

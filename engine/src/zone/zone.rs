use crate::coord_grid::CoordGrid;
use crate::zone::zone_event::{ZoneEvent, ZoneEventType};
use cache::ScriptZone;
use packet::out::model::map_anim::MapAnim;

pub struct Zone {
    pub index: u32,
    pub events: Vec<ZoneEvent>,
    pub total_locs: u16,
    pub total_objs: u8,
}

impl Zone {
    pub fn new(index: u32) -> Zone {
        return Zone {
            index,
            events: Vec::new(),
            total_locs: 0,
            total_objs: 0,
        };
    }

    pub fn tick(&self, current_tick: u32) {
        println!("zone: {}; tick {}", self.index, current_tick);
    }

    pub fn reset(&mut self) {
        self.events.clear();
    }
}

impl ScriptZone for Zone {
    fn index(&self) -> u32 {
        return self.index;
    }

    fn anim_map(&mut self, x: u16, y: u8, z: u16, spotanim: u16, height: i32, delay: u32) {
        self.events.push(ZoneEvent {
            zone_event_type: ZoneEventType::Enclosed,
            receiver_id: -1,
            message: Box::new(MapAnim {
                coord: CoordGrid::zone_coord(x, z),
                spotanim,
                height,
                delay,
            }),
        });
    }
}

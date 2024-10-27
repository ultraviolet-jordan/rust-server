use crate::zone::zone_event::{ZoneEvent, ZoneEventType};
use cache::ScriptZone;
use std::collections::HashSet;

pub struct Zone {
    pub index: u32,
    pub events: HashSet<ZoneEvent>,
    pub total_locs: u16,
    pub total_objs: u8,
}

impl Zone {
    pub fn new(index: u32) -> Zone {
        return Zone {
            index,
            events: HashSet::new(),
            total_locs: 0,
            total_objs: 0,
        };
    }
}

impl ScriptZone for Zone {
    fn anim_map(&mut self, y: u8, x: u16, z: u16, spotanim: u16, height: i32, delay: u32) {
        self.events.insert(ZoneEvent {
            zone_event_type: ZoneEventType::Enclosed,
            receiver_id: -1,
        });
    }
}

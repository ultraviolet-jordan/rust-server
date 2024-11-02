use crate::entity::lifetime::EntityLifetime;
use crate::entity::loc::Loc;
use crate::entity::obj::Obj;
use crate::grid::coord_grid::CoordGrid;
use crate::grid::zone::zone_event::{ZoneEvent, ZoneEventType, ZoneMessageType};
use crate::grid::zone::zone_map::ZoneMap;
use cache::ScriptZone;
use log::info;
use packet::out::model::loc_add_change::LocAddChange;
use packet::out::model::map_anim::MapAnim;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct Zone {
    pub index: u32,
    pub x: u16,
    pub y: u8,
    pub z: u16,
    pub players: HashSet<i32>,
    pub npcs: HashSet<i32>,
    pub locs: Vec<Rc<Loc>>,
    pub objs: Vec<Rc<Obj>>,
    pub entity_events: HashMap<Rc<Loc>, Vec<Rc<ZoneEvent>>>,
    pub events: HashSet<Rc<ZoneEvent>>,
    pub total_locs: u16,
    pub total_objs: u16,
}

impl Zone {
    const SIZE: usize = 8 * 8;
    const LOCS: usize = Zone::SIZE << 2;
    const OBJS: usize = (Zone::SIZE << 1) + 1;

    pub fn new(index: u32) -> Zone {
        let (x, y, z) = ZoneMap::unpack_index(index);
        return Zone {
            index,
            x: x >> 3,
            y,
            z: z >> 3,
            players: HashSet::new(),
            npcs: HashSet::new(),
            locs: Vec::with_capacity(Zone::LOCS), // TODO
            objs: Vec::with_capacity(Zone::OBJS), // TODO
            entity_events: HashMap::new(),
            events: HashSet::new(),
            total_locs: 0,
            total_objs: 0,
        };
    }

    pub fn tick(&self, current_tick: u32) {
        // TODO
        info!("zone: {}; tick {}", self.index, current_tick);
    }

    pub fn reset(&mut self) {
        // TODO
        self.events.clear();
    }

    // ---- static locs/objs are added during world init ----

    pub fn add_static_loc(&mut self, loc: Loc) {
        // TODO
        self.locs.push(Rc::new(loc));
        self.total_locs += 1;
    }

    pub fn add_static_obj(&mut self, obj: Obj) {
        // TODO
        self.objs.push(Rc::new(obj));
        self.total_objs += 1;
    }

    fn queue_event(&mut self, entity: Rc<Loc>, event: ZoneEvent) {
        let event: Rc<ZoneEvent> = Rc::new(event);
        if let Some(events) = self.entity_events.get_mut(&entity) {
            events.push(event.clone());
        } else {
            self.entity_events.insert(entity, vec![event.clone()]);
        }
        self.events.insert(event);
    }

    fn clear_queued_events(&mut self, entity: &Rc<Loc>) {
        if let Some(events) = self.entity_events.get(entity) {
            for event in events {
                self.events.remove(event);
            }
            self.entity_events.remove(entity);
        }
    }
}

impl ScriptZone for Zone {
    fn index(&self) -> u32 {
        return self.index;
    }

    fn anim_map(&mut self, x: u16, z: u16, spotanim: u16, height: i32, delay: u32) {
        self.events.insert(Rc::new(ZoneEvent {
            zone_event_type: ZoneEventType::Enclosed,
            receiver_id: -1,
            message: ZoneMessageType::MapAnim(MapAnim {
                coord: CoordGrid::from(x, self.y, z).zone_coord(),
                spotanim,
                height,
                delay,
            }),
        }));
    }

    fn add_loc(
        &mut self,
        x: u16,
        y: u8,
        z: u16,
        width: u8,
        length: u8,
        id: u16,
        shape: u8,
        angle: u8,
    ) {
        let loc: Rc<Loc> = Rc::new(Loc::new(
            CoordGrid::from(x, y, z),
            width,
            length,
            EntityLifetime::Despawn,
            id,
            shape,
            angle,
        ));

        let coord: u8 = loc.entity.coord.zone_coord();
        self.queue_event(
            loc.clone(),
            ZoneEvent {
                zone_event_type: ZoneEventType::Enclosed,
                receiver_id: -1,
                message: ZoneMessageType::LocAddChange(LocAddChange {
                    coord,
                    id,
                    shape,
                    angle,
                }),
            },
        );

        if loc.entity.lifetime == EntityLifetime::Despawn {
            self.locs.push(loc);
            self.total_locs += 1;
        }

        // TODO sorting
    }
}

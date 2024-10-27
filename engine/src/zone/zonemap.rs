use crate::zone::zone::Zone;
use crate::zone::zonegrid::ZoneGrid;
use std::collections::HashMap;

pub struct ZoneMap {
    pub zones: HashMap<u32, Zone>,
    pub grids: HashMap<u8, ZoneGrid>,
}

impl ZoneMap {
    #[inline(always)]
    pub fn zone_index(x: u16, z: u16, y: u8) -> u32 {
        return (((x >> 3) & 0x7ff) as u32)
            | ((((z >> 3) & 0x7ff) as u32) << 11)
            | (((y & 0x3) as u32) << 22);
    }

    pub fn new() -> ZoneMap {
        return ZoneMap {
            zones: HashMap::new(),
            grids: HashMap::new(),
        };
    }

    pub fn mock() -> ZoneMap {
        return ZoneMap {
            zones: HashMap::new(),
            grids: HashMap::new(),
        };
    }

    pub fn zone(&mut self, x: u16, z: u16, y: u8) -> &mut Zone {
        let zone_index: u32 = ZoneMap::zone_index(x, z, y);
        return self
            .zones
            .entry(zone_index)
            .or_insert(Zone::new(zone_index));
    }

    pub fn zone_by_index(&mut self, index: u32) -> &Zone {
        return self.zones.entry(index).or_insert(Zone::new(index));
    }

    pub fn grid(&mut self, y: u8) -> &mut ZoneGrid {
        return self.grids.entry(y).or_insert(ZoneGrid::new());
    }

    pub fn zone_count(&self) -> u32 {
        return self.zones.len() as u32;
    }

    pub fn loc_count(&self) -> u32 {
        let mut total: u32 = 0;
        for zone in self.zones.values() {
            total += zone.total_locs as u32;
        }
        return total;
    }

    pub fn obj_count(&self) -> u32 {
        let mut total: u32 = 0;
        for zone in self.zones.values() {
            total += zone.total_objs as u32;
        }
        return total;
    }
}

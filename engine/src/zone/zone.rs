pub struct Zone {
    pub index: u32,
    pub total_locs: u8,
    pub total_objs: u8,
}

impl Zone {
    pub fn new(index: u32) -> Zone {
        return Zone {
            index,
            total_locs: 0,
            total_objs: 0,
        };
    }
}

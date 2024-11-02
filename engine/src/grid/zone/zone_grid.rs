pub struct ZoneGrid {
    pub grid: Vec<u32>,
}

impl ZoneGrid {
    const GRID_SIZE: u16 = 2048;
    const INT_BITS: u8 = 5;
    const INT_BITS_FLAG: u8 = (1 << ZoneGrid::INT_BITS) - 1;
    const DEFAULT_GRID_SIZE: usize = ZoneGrid::GRID_SIZE as usize
        * (ZoneGrid::GRID_SIZE as usize >> ZoneGrid::INT_BITS as usize);

    pub fn new() -> ZoneGrid {
        return ZoneGrid {
            grid: vec![0; ZoneGrid::DEFAULT_GRID_SIZE],
        };
    }
}

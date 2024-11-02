pub struct CRC {
    pub table: Vec<i32>,
}

impl CRC {
    /// Reversed CRC-32 polynomial for Cyclic Redundancy Check (CRC).
    /// This is sometimes referred to as CRC32B.
    const CRC32B: u32 = 0xEDB88320;

    pub fn new() -> CRC {
        let mut table: Vec<i32> = vec![0; 256];
        for index in 0..256 {
            let mut remainder: u32 = index;
            for _ in 0..8 {
                if (remainder & 0x1) == 1 {
                    remainder = (remainder.wrapping_shr(1)) ^ CRC::CRC32B;
                } else {
                    remainder >>= 0x1;
                }
            }
            table[index as usize] = remainder as i32;
        }
        return CRC { table };
    }

    pub fn getcrc(&self, src: &Vec<u8>, offset: usize, length: usize) -> i32 {
        let mut crc: i32 = -1;
        for i in offset..length {
            crc = (crc >> 8) ^ (self.table[((crc ^ src[i] as i32) & 0xff) as usize]);
        }
        return !crc;
    }
}

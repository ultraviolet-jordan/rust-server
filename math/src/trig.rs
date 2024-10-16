pub struct Trig {
    sin: Vec<i32>,
    cos: Vec<i32>,
}

impl Trig {
    pub fn new() -> Trig {
        let mut sin: Vec<i32> = vec![0; 16384];
        let mut cos: Vec<i32> = vec![0; 16384];
        let size: f64 = 3.834951969714103E-4;
        for index in 0..16384 {
            let (s, c) = f64::sin_cos(index as f64 * size);
            sin[index] = (s * 16384.0) as i32;
            cos[index] = (c * 16384.0) as i32;
        }
        return Trig { sin, cos };
    }

    #[inline(always)]
    pub fn radians(x: i32) -> f64 {
        return (((x & 0x3fff) as f64) / 16384.0) * 6.283185307179586;
    }

    #[inline(always)]
    pub fn atan2(y: i32, x: i32) -> i32 {
        return ((f64::atan2(y as f64, x as f64) * 2607.5945876176133).round() as i32) & 0x3fff;
    }

    #[inline(always)]
    pub fn sin(&self, value: i32) -> i32 {
        return self.sin[(value & 0x3fff) as usize];
    }

    #[inline(always)]
    pub fn cos(&self, value: i32) -> i32 {
        return self.cos[(value & 0x3fff) as usize];
    }
}

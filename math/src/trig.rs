pub struct Trig {
    sin: Vec<i32>,
    cos: Vec<i32>,
}

impl Trig {
    pub fn new() -> Trig {
        let sin: Vec<i32> = vec![0; 16384];
        let cos: Vec<i32> = vec![0; 16384];
        return Trig { sin, cos };
    }
}

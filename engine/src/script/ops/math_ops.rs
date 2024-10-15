use rand::random;

use cache::{ScriptOpcode, ScriptState};
use math::bits::Bits;
use math::trig::Trig;

pub struct MathOps {
    trig: Trig,
    bits: Bits,
}

impl MathOps {
    pub fn new() -> MathOps {
        return MathOps {
            trig: Trig::new(),
            bits: Bits::new(),
        };
    }

    pub fn push(&self, state: &mut ScriptState, code: &ScriptOpcode) {
        match code {
            ScriptOpcode::Add => self.add(state),
            ScriptOpcode::Sub => self.sub(state),
            ScriptOpcode::Multiply => self.multiply(state),
            ScriptOpcode::Divide => self.divide(state),
            ScriptOpcode::Random => self.random(state),
            ScriptOpcode::RandomInc => self.randominc(state),
            ScriptOpcode::Interpolate => self.interpolate(state),
            ScriptOpcode::AddPercent => self.addpercent(state),
            ScriptOpcode::SetBit => self.setbit(state),
            ScriptOpcode::ClearBit => self.clearbit(state),
            ScriptOpcode::TestBit => self.testbit(state),
            ScriptOpcode::Modulo => self.modulo(state),
            ScriptOpcode::Pow => self.pow(state),
            ScriptOpcode::InvPow => self.invpow(state),
            ScriptOpcode::And => self.and(state),
            ScriptOpcode::Or => self.or(state),
            ScriptOpcode::Min => self.min(state),
            ScriptOpcode::Max => self.max(state),
            ScriptOpcode::Scale => self.scale(state),
            ScriptOpcode::BitCount => self.bitcount(state),
            ScriptOpcode::ToggleBit => self.togglebit(state),
            ScriptOpcode::SetBitRange => self.setbit_range(state),
            ScriptOpcode::ClearBitRange => self.clearbit_range(state),
            ScriptOpcode::GetBitRange => self.getbit_range(state),
            ScriptOpcode::SetBitRangeToInt => self.setbit_range_toint(state),
            ScriptOpcode::SinDeg => self.sin_deg(state),
            ScriptOpcode::CosDeg => self.cos_deg(state),
            ScriptOpcode::Atan2Deg => self.atan2_deg(state),
            ScriptOpcode::Abs => self.abs(state),
            _ => panic!("Unrecognised math ops code: {:?}", code),
        }
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(1);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Add);
    /// assert_eq!(3, state.pop_int());
    /// ```
    #[inline(always)]
    fn add(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a + b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(1);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Sub);
    /// assert_eq!(-1, state.pop_int());
    /// ```
    #[inline(always)]
    fn sub(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a - b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(1);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Multiply);
    /// assert_eq!(2, state.pop_int());
    /// ```
    #[inline(always)]
    fn multiply(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a * b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(50);
    /// state.push_int(5);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Divide);
    /// assert_eq!(10, state.pop_int());
    /// ```
    #[inline(always)]
    fn divide(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a / b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(100);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Random);
    /// let rand = state.pop_int();
    /// assert!(rand >= 0 && rand <= 99);
    /// ```
    #[inline(always)]
    fn random(&self, state: &mut ScriptState) {
        let a: f64 = state.pop_int() as f64;
        state.push_int((random::<f64>() * a) as i32);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(100);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::RandomInc);
    /// let rand = state.pop_int();
    /// assert!(rand >= 0 && rand <= 100);
    /// ```
    #[inline(always)]
    fn randominc(&self, state: &mut ScriptState) {
        let a: f64 = (state.pop_int() + 1) as f64;
        state.push_int((random::<f64>() * a) as i32);
    }

    #[inline(always)]
    fn interpolate(&self, state: &mut ScriptState) {
        let x: i32 = state.pop_int();
        let x1: i32 = state.pop_int();
        let x0: i32 = state.pop_int();
        let y1: i32 = state.pop_int();
        let y0: i32 = state.pop_int();
        let floor: f64 = ((y1 - y0) as f64 / (x1 - x0) as f64).floor();
        state.push_int((floor * (x - x0) as f64 + y0 as f64) as i32);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(50);
    /// state.push_int(6);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::AddPercent);
    /// assert_eq!(53, state.pop_int());
    /// ```
    #[inline(always)]
    fn addpercent(&self, state: &mut ScriptState) {
        let percent: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(num * percent / 100 + num);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(11);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::SetBit);
    /// assert_eq!(15, state.pop_int());
    /// ```
    #[inline(always)]
    fn setbit(&self, state: &mut ScriptState) {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(value | (1 << bit));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(15);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::ClearBit);
    /// assert_eq!(11, state.pop_int());
    /// ```
    #[inline(always)]
    fn clearbit(&self, state: &mut ScriptState) {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(value & !(1 << bit));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(15);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::TestBit);
    /// assert_eq!(1, state.pop_int());
    /// ```
    /// ----
    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(11);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::TestBit);
    /// assert_eq!(0, state.pop_int());
    /// ```
    #[inline(always)]
    fn testbit(&self, state: &mut ScriptState) {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(((value & (1 << bit)) != 0) as i32);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(41);
    /// state.push_int(6);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Modulo);
    /// assert_eq!(5, state.pop_int());
    /// ```
    #[inline(always)]
    fn modulo(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a % b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(100);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Pow);
    /// assert_eq!(10000, state.pop_int());
    /// ```
    #[inline(always)]
    fn pow(&self, state: &mut ScriptState) {
        let exponent: i32 = state.pop_int();
        let base: i32 = state.pop_int();
        state.push_int(base.pow(exponent as u32));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(100);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::InvPow);
    /// assert_eq!(10, state.pop_int());
    /// ```
    #[inline(always)]
    fn invpow(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a == 0 || b == 0 {
            state.push_int(0);
            return;
        }
        match b {
            1 => state.push_int(a),
            2 => state.push_int((a as f64).sqrt() as i32),
            3 => state.push_int((a as f64).cbrt() as i32),
            4 => state.push_int((a as f64).sqrt().sqrt() as i32),
            _ => state.push_int(a.pow((1.0 / b as f64) as u32)),
        }
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(46);
    /// state.push_int(33);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::And);
    /// assert_eq!(32, state.pop_int());
    /// ```
    #[inline(always)]
    fn and(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a & b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(46);
    /// state.push_int(533);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Or);
    /// assert_eq!(575, state.pop_int());
    /// ```
    #[inline(always)]
    fn or(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a | b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(46);
    /// state.push_int(533);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Min);
    /// assert_eq!(46, state.pop_int());
    /// ```
    #[inline(always)]
    fn min(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.min(b));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(46);
    /// state.push_int(533);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Max);
    /// assert_eq!(533, state.pop_int());
    /// ```
    #[inline(always)]
    fn max(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(a.max(b));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(46);
    /// state.push_int(533);
    /// state.push_int(69);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Scale);
    /// assert_eq!(5, state.pop_int());
    /// ```
    #[inline(always)]
    fn scale(&self, state: &mut ScriptState) {
        let c: i32 = state.pop_int();
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int((a * c) / b);
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(15);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::BitCount);
    /// assert_eq!(4, state.pop_int());
    /// ```
    #[inline(always)]
    fn bitcount(&self, state: &mut ScriptState) {
        let a: i32 = state.pop_int();
        state.push_int(Bits::bitcount(a));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(11);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::ToggleBit);
    /// assert_eq!(15, state.pop_int());
    /// ```
    #[inline(always)]
    fn togglebit(&self, state: &mut ScriptState) {
        let bit: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        state.push_int(value ^ (1 << bit));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(0);
    /// state.push_int(1);
    /// state.push_int(3);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::SetBitRange);
    /// assert_eq!(14, state.pop_int());
    /// ```
    #[inline(always)]
    fn setbit_range(&self, state: &mut ScriptState) {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(self.bits.setbit_range(num, start, end));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(15);
    /// state.push_int(1);
    /// state.push_int(3);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::ClearBitRange);
    /// assert_eq!(1, state.pop_int());
    /// ```
    #[inline(always)]
    fn clearbit_range(&self, state: &mut ScriptState) {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(self.bits.clearbit_range(num, start, end));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(15);
    /// state.push_int(0);
    /// state.push_int(2);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::GetBitRange);
    /// assert_eq!(7, state.pop_int());
    /// ```
    #[inline(always)]
    fn getbit_range(&self, state: &mut ScriptState) {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        let r: i32 = 31 - end;
        state.push_int((((num << r) as u32) >> ((start + r) as u32)) as i32)
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(0);
    /// state.push_int(3);
    /// state.push_int(1);
    /// state.push_int(3);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::SetBitRangeToInt);
    /// assert_eq!(6, state.pop_int());
    /// ```
    #[inline(always)]
    fn setbit_range_toint(&self, state: &mut ScriptState) {
        let end: i32 = state.pop_int();
        let start: i32 = state.pop_int();
        let value: i32 = state.pop_int();
        let num: i32 = state.pop_int();
        state.push_int(self.bits.setbit_range_toint(num, value, start, end));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(std::f64::consts::PI as i32);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::SinDeg);
    /// assert_eq!(18, state.pop_int());
    /// ```
    #[inline(always)]
    fn sin_deg(&self, state: &mut ScriptState) {
        let a: i32 = state.pop_int();
        state.push_int(self.trig.sin(a));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(std::f64::consts::PI as i32);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::CosDeg);
    /// assert_eq!(16383, state.pop_int());
    /// ```
    #[inline(always)]
    fn cos_deg(&self, state: &mut ScriptState) {
        let a: i32 = state.pop_int();
        state.push_int(self.trig.cos(a));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(-1);
    /// state.push_int(1);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Atan2Deg);
    /// assert_eq!(6144, state.pop_int());
    /// ```
    #[inline(always)]
    fn atan2_deg(&self, state: &mut ScriptState) {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        state.push_int(Trig::atan2(b, a));
    }

    /// ```rust
    /// use cache::{ScriptFile, ScriptOpcode, ScriptState};
    /// use engine::script::ops::math_ops::MathOps;
    ///
    /// let file = ScriptFile::mock();
    /// let mut state = ScriptState::mock(&file);
    /// state.push_int(-136);
    ///
    /// let ops = MathOps::new();
    /// ops.push(&mut state, &ScriptOpcode::Abs);
    /// assert_eq!(136, state.pop_int());
    /// ```
    #[inline(always)]
    fn abs(&self, state: &mut ScriptState) {
        let a: i32 = state.pop_int();
        state.push_int(a.abs());
    }
}

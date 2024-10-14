use cache::{ScriptOpcode, ScriptState};
use math::trig::Trig;

pub struct MathOps {
    trig: Trig,
}

impl MathOps {
    pub fn new() -> MathOps {
        return MathOps { trig: Trig::new() };
    }

    pub fn push(&self, state: &mut ScriptState, code: &ScriptOpcode) {
        match code {
            ScriptOpcode::Add => {
                let b: i32 = state.pop_int();
                let a: i32 = state.pop_int();
                state.push_int(a + b);
            }
            ScriptOpcode::Sub => panic!("Not implemented"),
            ScriptOpcode::Multiply => panic!("Not implemented"),
            ScriptOpcode::Divide => panic!("Not implemented"),
            ScriptOpcode::Random => panic!("Not implemented"),
            ScriptOpcode::RandomInc => panic!("Not implemented"),
            ScriptOpcode::Interpolate => panic!("Not implemented"),
            ScriptOpcode::AddPercent => panic!("Not implemented"),
            ScriptOpcode::SetBit => panic!("Not implemented"),
            ScriptOpcode::ClearBit => panic!("Not implemented"),
            ScriptOpcode::TestBit => panic!("Not implemented"),
            ScriptOpcode::Modulo => panic!("Not implemented"),
            ScriptOpcode::Pow => panic!("Not implemented"),
            ScriptOpcode::InvPow => panic!("Not implemented"),
            ScriptOpcode::And => panic!("Not implemented"),
            ScriptOpcode::Or => panic!("Not implemented"),
            ScriptOpcode::Min => panic!("Not implemented"),
            ScriptOpcode::Max => panic!("Not implemented"),
            ScriptOpcode::Scale => panic!("Not implemented"),
            ScriptOpcode::BitCount => panic!("Not implemented"),
            ScriptOpcode::ToggleBit => panic!("Not implemented"),
            ScriptOpcode::SetBitRange => panic!("Not implemented"),
            ScriptOpcode::ClearBitRange => panic!("Not implemented"),
            ScriptOpcode::GetBitRange => panic!("Not implemented"),
            ScriptOpcode::SetBitRangeToInt => panic!("Not implemented"),
            ScriptOpcode::SinDeg => panic!("Not implemented"),
            ScriptOpcode::CosDeg => panic!("Not implemented"),
            ScriptOpcode::Atan2Deg => panic!("Not implemented"),
            ScriptOpcode::Abs => panic!("Not implemented"),
            _ => panic!("Unrecognised math ops code: {:?}", code),
        }
    }
}

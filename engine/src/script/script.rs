use crate::script::ops::core_ops::CoreOps;
use crate::script::ops::math_ops::MathOps;
use crate::script::ops::oc_ops::OcOps;
use crate::script::ops::string_ops::StringOps;

pub struct Ops {
    pub core: CoreOps,
    pub math: MathOps,
    pub oc: OcOps,
    pub string: StringOps,
}

impl Ops {
    pub fn new() -> Ops {
        return Ops {
            core: CoreOps::new(),
            math: MathOps::new(),
            oc: OcOps::new(),
            string: StringOps::new(),
        };
    }
}

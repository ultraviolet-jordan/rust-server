use cache::{ObjType, ScriptEngine, ScriptOpcode, ScriptState};

pub struct OcOps;

impl OcOps {
    pub fn new() -> OcOps {
        return OcOps;
    }

    pub fn push(
        &self,
        engine: &impl ScriptEngine,
        state: &mut ScriptState,
        code: &ScriptOpcode,
    ) -> Result<(), String> {
        match code {
            ScriptOpcode::OcCategory => panic!("Not implemented"),
            ScriptOpcode::OcCert => panic!("Not implemented"),
            ScriptOpcode::OcCost => panic!("Not implemented"),
            ScriptOpcode::OcDebugname => panic!("Not implemented"),
            ScriptOpcode::OcDesc => panic!("Not implemented"),
            ScriptOpcode::OcIop => panic!("Not implemented"),
            ScriptOpcode::OcMembers => panic!("Not implemented"),
            ScriptOpcode::OcName => self.oc_name(engine, state),
            ScriptOpcode::OcOp => panic!("Not implemented"),
            ScriptOpcode::OcParam => panic!("Not implemented"),
            ScriptOpcode::OcStackable => panic!("Not implemented"),
            ScriptOpcode::OcTradeable => panic!("Not implemented"),
            ScriptOpcode::OcUncert => panic!("Not implemented"),
            ScriptOpcode::OcWearPos2 => panic!("Not implemented"),
            ScriptOpcode::OcWearPos3 => panic!("Not implemented"),
            ScriptOpcode::OcWearPos => panic!("Not implemented"),
            ScriptOpcode::OcWeight => panic!("Not implemented"),
            _ => Err(format!("Unrecognised oc ops code: {:?}", code)),
        }
    }

    #[inline(always)]
    fn oc_name(&self, engine: &impl ScriptEngine, state: &mut ScriptState) -> Result<(), String> {
        let obj: &ObjType = engine.pop_obj(state.pop_int())?;
        state.push_string(
            obj.name
                .as_ref()
                .or(obj.debugname.as_ref())
                .unwrap_or(&String::new())
                .clone(),
        );
        return Ok(());
    }
}

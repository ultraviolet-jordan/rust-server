use cache::{ScriptEngine, ScriptOpcode, ScriptState};

pub struct OcOps;

impl OcOps {
    pub fn new() -> OcOps {
        return OcOps;
    }

    pub fn push(&self, engine: &impl ScriptEngine, state: &mut ScriptState, code: &ScriptOpcode) {
        match code {
            ScriptOpcode::OcCategory => panic!("Not implemented"),
            ScriptOpcode::OcCert => panic!("Not implemented"),
            ScriptOpcode::OcCost => panic!("Not implemented"),
            ScriptOpcode::OcDebugname => panic!("Not implemented"),
            ScriptOpcode::OcDesc => panic!("Not implemented"),
            ScriptOpcode::OcIop => panic!("Not implemented"),
            ScriptOpcode::OcMembers => panic!("Not implemented"),
            ScriptOpcode::OcName => match engine.pop_obj(state.pop_int()) {
                Ok(obj) => {
                    if let Some(name) = &obj.name {
                        state.push_string(name.clone());
                    } else if let Some(debugname) = &obj.debugname {
                        state.push_string(debugname.clone());
                    } else {
                        state.push_string(String::new());
                    }
                }
                Err(e) => panic!("{}", e),
            },
            ScriptOpcode::OcOp => panic!("Not implemented"),
            ScriptOpcode::OcParam => panic!("Not implemented"),
            ScriptOpcode::OcStackable => panic!("Not implemented"),
            ScriptOpcode::OcTradeable => panic!("Not implemented"),
            ScriptOpcode::OcUncert => panic!("Not implemented"),
            ScriptOpcode::OcWearPos2 => panic!("Not implemented"),
            ScriptOpcode::OcWearPos3 => panic!("Not implemented"),
            ScriptOpcode::OcWearPos => panic!("Not implemented"),
            ScriptOpcode::OcWeight => panic!("Not implemented"),
            _ => panic!("Unrecognised oc ops code: {:?}", code),
        }
    }
}

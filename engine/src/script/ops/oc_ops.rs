use cache::{CacheProvider, ScriptOpcode, ScriptState};

pub fn script_oc_ops<'script>(
    code: &ScriptOpcode,
    state: &mut ScriptState<'script>,
    provider: &'script CacheProvider,
) {
    match code {
        ScriptOpcode::OcCategory => panic!("Not implemented"),
        ScriptOpcode::OcCert => panic!("Not implemented"),
        ScriptOpcode::OcCost => panic!("Not implemented"),
        ScriptOpcode::OcDebugname => panic!("Not implemented"),
        ScriptOpcode::OcDesc => panic!("Not implemented"),
        ScriptOpcode::OcIop => panic!("Not implemented"),
        ScriptOpcode::OcMembers => panic!("Not implemented"),
        ScriptOpcode::OcName => {
            let obj: usize = state.pop_int() as usize;
            provider.objs.get_by_id(
                obj,
                |obj| {
                    if let Some(name) = &obj.name {
                        state.push_string(name.clone());
                    } else if let Some(debugname) = &obj.name {
                        state.push_string(debugname.clone());
                    } else {
                        state.push_string(String::new());
                    }
                },
                || panic!("[ScriptOpcode::OcName] Obj not found for pop_int: {}", obj),
            )
        }
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

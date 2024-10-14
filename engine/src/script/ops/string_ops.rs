use cache::{ScriptOpcode, ScriptState};

pub fn script_string_ops(state: &mut ScriptState, code: &ScriptOpcode) {
    match code {
        ScriptOpcode::AppendNum => panic!("Not implemented"),
        ScriptOpcode::Append => panic!("Not implemented"),
        ScriptOpcode::AppendSignNum => panic!("Not implemented"),
        ScriptOpcode::Lowercase => panic!("Not implemented"),
        ScriptOpcode::TextGender => panic!("Not implemented"),
        ScriptOpcode::ToString => {
            let int: i32 = state.pop_int();
            state.push_string(int.to_string());
        }
        ScriptOpcode::Compare => panic!("Not implemented"),
        ScriptOpcode::TextSwitch => panic!("Not implemented"),
        ScriptOpcode::AppendChar => panic!("Not implemented"),
        ScriptOpcode::StringLength => panic!("Not implemented"),
        ScriptOpcode::SubString => panic!("Not implemented"),
        ScriptOpcode::StringIndexOfChar => panic!("Not implemented"),
        ScriptOpcode::StringIndexOfString => panic!("Not implemented"),
        _ => panic!("Unrecognised string ops code: {:?}", code),
    }
}

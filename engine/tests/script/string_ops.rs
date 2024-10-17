use cache::{ScriptFile, ScriptOpcode, ScriptState};
use engine::script::ops::string_ops::StringOps;

#[test]
fn test_to_string() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(420);

    let ops = StringOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::ToString);
    assert_eq!("420", state.pop_string());
    assert!(result.is_ok());
}

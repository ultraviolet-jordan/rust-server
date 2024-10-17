use cache::{ScriptFile, ScriptOpcode, ScriptRunner, ScriptState};
use engine::engine::Engine;

#[test]
fn test_to_string() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(420);

    let engine = Engine::mock();
    let result = engine.push_script(&mut state, &ScriptOpcode::ToString);
    assert_eq!("420", state.pop_string());
    assert!(result.is_ok());
}

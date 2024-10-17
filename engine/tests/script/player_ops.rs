use cache::{ScriptFile, ScriptOpcode, ScriptRunner, ScriptState};
use engine::engine::Engine;
use engine::entity::player::Player;

#[test]
pub fn test_bas_readyanim() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(0);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(69);

    let mut engine = Engine::mock();
    engine.add_player(0, Player::new());
    state.set_active_player(0);
    let result = engine.push_script(&mut state, &ScriptOpcode::BasReadyAnim);
    assert_eq!(69, engine.get_player(0).unwrap().bas_readyanim);
    assert!(result.is_ok());
}

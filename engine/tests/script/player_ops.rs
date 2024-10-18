use cache::{ScriptFile, ScriptOpcode, ScriptPointer, ScriptRunner, ScriptState};
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

#[test]
pub fn test_anim_no_access() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.

    let engine = Engine::mock();
    let result = engine.push_script(&mut state, &ScriptOpcode::Anim);
    assert!(result.is_err());
}

#[test]
pub fn test_anim_with_access() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(0);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(69);
    state.push_int(420);

    let mut engine = Engine::mock();
    engine.add_player(0, Player::new());
    state.set_active_player(0);
    state.pointer_add(ScriptPointer::ActivePlayer);
    let result = engine.push_script(&mut state, &ScriptOpcode::Anim);
    assert!(result.is_ok());
}

#[test]
pub fn test_find_uid_true() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(0);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(0);

    let mut engine = Engine::mock();
    engine.add_player(0, Player::new());
    let result = engine.push_script(&mut state, &ScriptOpcode::FindUid);
    assert!(result.is_ok());
    assert_eq!(0, state.get_active_player());
    assert_eq!(1, state.pop_int());
}

#[test]
pub fn test_find_uid_false() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(0);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(0);

    let engine = Engine::mock();
    let result = engine.push_script(&mut state, &ScriptOpcode::FindUid);
    assert!(result.is_ok());
    assert_eq!(-1, state.get_active_player());
    assert_eq!(0, state.pop_int());
}

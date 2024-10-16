use cache::{ScriptExecutionState, ScriptFile, ScriptOpcode, ScriptState};
use engine::engine::Engine;
use engine::script::ops::core_ops::CoreOps;

#[test]
fn test_push_constant_int() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::PushConstantInt);
    assert_eq!(1, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_push_constant_string() {
    let mut file = ScriptFile::mock();
    file.string_operands.push("Hello World!".to_string());
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::PushConstantString);
    assert_eq!("Hello World!", state.pop_string());
    assert!(result.is_ok());
}

#[test]
fn test_branch() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::Branch);
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_not_1() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(2);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchNot);
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_not_0() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchNot);
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_equals_0() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(2);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchEquals);
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_equals_1() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchEquals);
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_less_than_1() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(2);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchLessThan);
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_less_than_0() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchLessThan);
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_greater_than_0() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(2);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchGreaterThan);
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_greater_than_1() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(2);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchGreaterThan);
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_return_finished() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::Return);
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
    assert_eq!(ScriptExecutionState::Finished, state.execution_state);
}

#[test]
fn test_return_running() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    let script2 = ScriptFile::mock();
    state.push_frame(&script2);
    assert_eq!(-1, state.pc);
    assert_eq!(1, state.fp);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::Return);
    assert!(result.is_ok());
    assert_eq!(0, state.fp);
    assert_eq!(ScriptExecutionState::Running, state.execution_state);
}

#[test]
fn test_branch_less_than_or_equals_1() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchLessThanOrEquals);
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_less_than_or_equals_0() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(2);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::BranchLessThanOrEquals);
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_greater_than_or_equals_1() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(1);
    state.push_int(2);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(
        &engine,
        &mut state,
        &ScriptOpcode::BranchGreaterThanOrEquals,
    );
    assert_eq!(0, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_branch_greater_than_or_equals_0() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(1);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(2);
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(
        &engine,
        &mut state,
        &ScriptOpcode::BranchGreaterThanOrEquals,
    );
    assert_eq!(1, state.pc);
    assert!(result.is_ok());
}

#[test]
fn test_push_int_local() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(0);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.int_locals.push(1);
    state.pc += 1; // emulate starting the script program.

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::PushIntLocal);
    assert_eq!(1, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_pop_int_local() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(0);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.int_locals.push(0);
    state.pc += 1; // emulate starting the script program.
    state.push_int(1);

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::PopIntLocal);
    assert_eq!(1, state.int_locals[0]);
    assert!(result.is_ok());
}

#[test]
fn test_join_string() {
    let mut file = ScriptFile::mock();
    file.int_operands.push(2);
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_string("Hello".to_string());
    state.push_string("World!".to_string());

    let ops = CoreOps::new();
    let engine = Engine::mock();
    let result = ops.push(&engine, &mut state, &ScriptOpcode::JoinString);
    assert_eq!("HelloWorld!", state.pop_string());
    assert!(result.is_ok());
}

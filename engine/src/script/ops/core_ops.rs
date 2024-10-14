use cache::{ScriptEngine, ScriptExecutionState, ScriptOpcode, ScriptState};

pub fn script_core_ops<'script>(
    engine: &'script impl ScriptEngine,
    state: &mut ScriptState<'script>,
    code: &ScriptOpcode,
) {
    match code {
        ScriptOpcode::PushConstantInt => {
            state.push_int(state.int_operand());
        }
        ScriptOpcode::PushVarp => panic!("Not implemented"),
        ScriptOpcode::PopVarp => panic!("Not implemented"),
        ScriptOpcode::PushConstantString => {
            state.push_string(state.string_operand());
        }
        ScriptOpcode::PushVarn => panic!("Not implemented"),
        ScriptOpcode::PopVarn => panic!("Not implemented"),
        ScriptOpcode::Branch => {
            state.pc += state.int_operand();
        }
        ScriptOpcode::BranchNot => {
            let b: i32 = state.pop_int();
            let a: i32 = state.pop_int();
            if a != b {
                state.pc += state.int_operand();
            }
        }
        ScriptOpcode::BranchEquals => {
            let b: i32 = state.pop_int();
            let a: i32 = state.pop_int();
            if a == b {
                state.pc += state.int_operand();
            }
        }
        ScriptOpcode::BranchLessThan => {
            let b: i32 = state.pop_int();
            let a: i32 = state.pop_int();
            if a < b {
                state.pc += state.int_operand();
            }
        }
        ScriptOpcode::BranchGreaterThan => {
            let b: i32 = state.pop_int();
            let a: i32 = state.pop_int();
            if a > b {
                state.pc += state.int_operand();
            }
        }
        ScriptOpcode::PushVars => panic!("Not implemented"),
        ScriptOpcode::PopVars => panic!("Not implemented"),
        ScriptOpcode::Return => {
            if state.fp == 0 {
                state.execution_state = ScriptExecutionState::Finished;
                return;
            }
            state.pop_frame();
        }
        ScriptOpcode::GoSub => panic!("Not implemented"),
        ScriptOpcode::Jump => panic!("Not implemented"),
        ScriptOpcode::Switch => panic!("Not implemented"),
        ScriptOpcode::PushVarbit => panic!("Not implemented"),
        ScriptOpcode::PopVarbit => panic!("Not implemented"),
        ScriptOpcode::BranchLessThanOrEquals => {
            let b: i32 = state.pop_int();
            let a: i32 = state.pop_int();
            if a <= b {
                state.pc += state.int_operand();
            }
        }
        ScriptOpcode::BranchGreaterThanOrEquals => {
            let b: i32 = state.pop_int();
            let a: i32 = state.pop_int();
            if a >= b {
                state.pc += state.int_operand();
            }
        }
        ScriptOpcode::PushIntLocal => {
            state.push_int(state.int_locals[state.int_operand() as usize]);
        }
        ScriptOpcode::PopIntLocal => {
            let operand: usize = state.int_operand() as usize;
            state.int_locals[operand] = state.pop_int();
        }
        ScriptOpcode::PushStringLocal => panic!("Not implemented"),
        ScriptOpcode::PopStringLocal => panic!("Not implemented"),
        ScriptOpcode::JoinString => {
            let count: usize = state.int_operand() as usize;
            let mut result: String = String::new();
            for _ in 0..count {
                result = state.pop_string() + &result;
            }
            state.push_string(result);
        }
        ScriptOpcode::PopIntDiscard => {
            state.isp -= 1;
        }
        ScriptOpcode::PopStringDiscard => {
            state.ssp -= 1;
        }
        ScriptOpcode::GoSubWithParams => match engine.pop_script(state.int_operand()) {
            Ok(script) => state.push_frame(script),
            Err(e) => panic!("{}", e),
        },
        ScriptOpcode::JumpWithParams => panic!("Not implemented"),
        ScriptOpcode::PushVarcInt => panic!("Not implemented"),
        ScriptOpcode::PopVarcInt => panic!("Not implemented"),
        ScriptOpcode::DefineArray => panic!("Not implemented"),
        ScriptOpcode::PushArrayInt => panic!("Not implemented"),
        ScriptOpcode::PopArrayInt => panic!("Not implemented"),
        _ => panic!("Unrecognised core ops code: {:?}", code),
    }
}

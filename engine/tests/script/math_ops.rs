use cache::{ScriptFile, ScriptOpcode, ScriptState};
use engine::script::ops::math_ops::MathOps;

#[test]
fn test_add() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(1);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Add);
    assert_eq!(3, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_sub() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(1);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Sub);
    assert_eq!(-1, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_multiply() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(1);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Multiply);
    assert_eq!(2, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_divide() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(50);
    state.push_int(5);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Divide);
    assert_eq!(10, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_random() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(100);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Random);
    let rand = state.pop_int();
    assert!(rand >= 0 && rand <= 99);
    assert!(result.is_ok());
}

#[test]
fn test_randominc() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(100);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::RandomInc);
    let rand = state.pop_int();
    assert!(rand >= 0 && rand <= 100);
    assert!(result.is_ok());
}

#[test]
fn test_addpercent() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(50);
    state.push_int(6);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::AddPercent);
    assert_eq!(53, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_setbit() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(11);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::SetBit);
    assert_eq!(15, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_clearbit() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(15);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::ClearBit);
    assert_eq!(11, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_testbit_1() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(15);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::TestBit);
    assert_eq!(1, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_testbit_0() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(11);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::TestBit);
    assert_eq!(0, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_modulo() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(41);
    state.push_int(6);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Modulo);
    assert_eq!(5, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_pow() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(100);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Pow);
    assert_eq!(10000, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_invpow() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(100);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::InvPow);
    assert_eq!(10, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_and() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(46);
    state.push_int(33);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::And);
    assert_eq!(32, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_or() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(46);
    state.push_int(533);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Or);
    assert_eq!(575, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_min() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(46);
    state.push_int(533);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Min);
    assert_eq!(46, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_max() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(46);
    state.push_int(533);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Max);
    assert_eq!(533, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_scale() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(46);
    state.push_int(533);
    state.push_int(69);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Scale);
    assert_eq!(5, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_bitcount() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(15);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::BitCount);
    assert_eq!(4, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_togglebit() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(11);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::ToggleBit);
    assert_eq!(15, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_setbit_range() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(0);
    state.push_int(1);
    state.push_int(3);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::SetBitRange);
    assert_eq!(14, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_clearbit_range() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(15);
    state.push_int(1);
    state.push_int(3);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::ClearBitRange);
    assert_eq!(1, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_getbit_range() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(15);
    state.push_int(0);
    state.push_int(2);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::GetBitRange);
    assert_eq!(7, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_setbit_range_toint() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(0);
    state.push_int(3);
    state.push_int(1);
    state.push_int(3);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::SetBitRangeToInt);
    assert_eq!(6, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_sin_deg() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(std::f64::consts::PI as i32);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::SinDeg);
    assert_eq!(18, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_cos_deg() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(std::f64::consts::PI as i32);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::CosDeg);
    assert_eq!(16383, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_atan2_deg() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(-1);
    state.push_int(1);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Atan2Deg);
    assert_eq!(6144, state.pop_int());
    assert!(result.is_ok());
}

#[test]
fn test_abs() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    state.push_int(-136);

    let ops = MathOps::new();
    let result = ops.push(&mut state, &ScriptOpcode::Abs);
    assert_eq!(136, state.pop_int());
    assert!(result.is_ok());
}

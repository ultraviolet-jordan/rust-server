use cache::{ObjType, ScriptFile, ScriptOpcode, ScriptState};
use engine::engine::Engine;
use engine::script::ops::oc_ops::OcOps;

#[test]
fn test_oc_name() {
    let file = ScriptFile::mock();
    let mut state = ScriptState::mock(&file);
    assert_eq!(-1, state.pc);

    state.pc += 1; // emulate starting the script program.
    state.push_int(0);

    let ops = OcOps::new();
    let mut engine = Engine::mock();
    engine.cache.obj_provider.objs.push(Some(ObjType {
        id: 0,
        model: 0,
        name: Some("Hello World!".to_string()),
        desc: None,
        zoom2d: 2000,
        xan2d: 0,
        yan2d: 0,
        xof2d: 0,
        yof2d: 0,
        code9: false,
        code10: None,
        stackable: false,
        cost: 1,
        wearpos: None,
        wearpos2: None,
        members: false,
        manwear: None,
        manwear2: None,
        manweary: 0,
        womanwear: None,
        womanwear2: None,
        womanweary: 0,
        wearpos3: None,
        op: None,
        iop: None,
        recol_s: None,
        recol_d: None,
        weight: 0,
        manwear3: None,
        womanwear3: None,
        manhead: None,
        manhead2: None,
        womanhead: None,
        womanhead2: None,
        category: None,
        zan2d: 0,
        dummyitem: 0,
        certlink: None,
        certtemplate: None,
        countobj: None,
        countco: None,
        tradeable: false,
        respawnrate: 100,
        params: None,
        debugname: None,
    }));
    let result = ops.push(&engine, &mut state, &ScriptOpcode::OcName);
    assert_eq!("Hello World!", state.pop_string());
    assert!(result.is_ok());
}

use dotenv::dotenv;

use cache::{CacheProvider, ScriptPointer, ScriptState};
use engine::engine::Engine;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();

    let engine: Engine = Engine::new(CacheProvider::new(
        "./data/pack",
        std::env::var("COMPILER_VERSION").unwrap(),
        std::env::var("MEMBERS").unwrap() == "true",
    ));

    engine.cache.obj_provider.with_script_name(
        "christmas_cracker",
        |obj| {
            if let Some(desc) = &obj.desc {
                println!("{}", desc);
            }
        },
        || {},
    );

    engine.cache.script_provider.with_script_name(
        "[proc,fib]",
        |script| {
            let mut state: ScriptState = ScriptState::new_with_args(script, vec![45], Vec::new());
            state.pointer_add(ScriptPointer::ProtectedActivePlayer);
            match state.execute(&engine, false) {
                Ok(()) => {
                    println!(
                        "fib: result={}, opcount={}, pointers={}",
                        state.pop_int(),
                        state.opcount,
                        state.pointer_debug()
                    );
                }
                Err(e) => panic!("{}", e),
            };
        },
        || {},
    );

    engine.cache.script_provider.get_by_trigger(
        139,
        708,
        -1,
        |script| {
            println!("{}", script.info.name);
        },
        || {},
    );

    engine.cache.script_provider.with_script_name(
        "[proc,get_obj_name]",
        |script| {
            engine.cache.obj_provider.with_script_name(
                "christmas_cracker",
                |obj| {
                    let mut state: ScriptState =
                        ScriptState::new_with_args(script, vec![obj.id as i32], Vec::new());
                    match state.execute(&engine, false) {
                        Ok(()) => {
                            println!(
                                "get_obj_name: result={}, opcount={}",
                                state.pop_string(),
                                state.opcount
                            );
                        }
                        Err(e) => panic!("{}", e),
                    };
                },
                || {},
            );
        },
        || {},
    );

    engine.cache.script_provider.with_script_name(
        "[proc,test_jump]",
        |script| {
            let mut state: ScriptState = ScriptState::new_with_args(script, Vec::new(), Vec::new());
            let _ = state.execute(&engine, false);
        },
        || {},
    );
}

use dotenv::dotenv;

use cache::{CacheProvider, ScriptPointer, ScriptState};
use engine::engine::Engine;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();

    let engine: Engine = Engine::new(CacheProvider::new(
        "./data/pack",
        std::env::var("MEMBERS").unwrap() == "true",
    ));

    engine.cache.objs.on_by_name(
        "christmas_cracker",
        |obj| {
            if let Some(desc) = &obj.desc {
                println!("{}", desc);
            }
        },
        || {},
    );

    engine.cache.scripts.on_by_name(
        "[proc,fib]",
        |script| {
            let mut state: ScriptState = ScriptState::new_with_args(script, vec![45], Vec::new());
            state.execute(&engine, false);
            state.pointer_add(ScriptPointer::ProtectedActivePlayer);
            println!(
                "fib: result={}, opcount={}, pointers={}",
                state.pop_int(),
                state.opcount,
                state.pointer_debug()
            );
        },
        || {},
    );

    engine.cache.scripts.get_by_trigger(
        139,
        708,
        -1,
        |script| {
            println!("{}", script.info.name);
        },
        || {},
    );

    engine.cache.scripts.on_by_name(
        "[proc,get_obj_name]",
        |script| {
            engine.cache.objs.on_by_name(
                "christmas_cracker",
                |obj| {
                    let mut state: ScriptState =
                        ScriptState::new_with_args(script, vec![obj.id as i32], Vec::new());
                    state.execute(&engine, false);
                    println!(
                        "get_obj_name: result={}, opcount={}",
                        state.pop_string(),
                        state.opcount
                    );
                },
                || {},
            );
        },
        || {},
    );
}

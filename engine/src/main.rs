use dotenv::dotenv;

use cache::{ObjProvider, ScriptPointer, ScriptProvider, ScriptState};
use engine::script::script_runner;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();
    let obj_provider: ObjProvider =
        ObjProvider::io("./data/pack", std::env::var("MEMBERS").unwrap() == "true");
    let script_provider: ScriptProvider = ScriptProvider::io("./data/pack");

    obj_provider.get_by_name(
        "christmas_cracker",
        |obj| {
            if let Some(desc) = &obj.desc {
                println!("{}", desc);
            }
        },
        || {},
    );

    script_provider.get_by_name(
        "[proc,fib]",
        |script| {
            let mut state: ScriptState = ScriptState::new_with_args(script, vec![45], Vec::new());
            state.execute(&script_provider, script_runner);
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

    script_provider.get_by_trigger(
        139,
        708,
        -1,
        |script| {
            println!("{}", script.info.name);
        },
        || {},
    );
}

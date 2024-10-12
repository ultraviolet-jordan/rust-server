use dotenv::dotenv;

use cache::{ObjProvider, ScriptProvider};

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
        "[opnpc1,helemos]",
        |script| {
            if let Some(info) = &script.info {
                println!("{}", info.path);
            }
        },
        || {},
    );
}

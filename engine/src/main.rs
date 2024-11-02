use dotenv::dotenv;
use std::process::Command;

use cache::CacheProvider;
use engine::engine::Engine;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();

    let data: &str = "./data/pack";
    let members: bool = std::env::var("NODE_MEMBERS").unwrap() == "true";

    // io load cache
    let cache_provider: CacheProvider =
        CacheProvider::io(data, std::env::var("COMPILER_VERSION").unwrap(), members);

    // start the web server.
    if let Err(e) = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("web_server")
        .arg("--release")
        .spawn()
    {
        eprintln!("Failed to start WEB server: {}", e);
    }

    // create & start engine
    Engine::new(cache_provider, members).start(true);
}

use cache::CacheProvider;
use dotenv::dotenv;
use engine::engine::Engine;
use log::error;
use std::process::Command;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();
    pretty_env_logger::init();

    let data: &str = "./data/pack";
    let members: bool = std::env::var("NODE_MEMBERS").unwrap() == "true";

    // io load cache
    let cache_provider: CacheProvider =
        CacheProvider::io(data, std::env::var("COMPILER_VERSION").unwrap(), members);

    // start the web server.
    if let Err(e) = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("web-server")
        .arg("--release")
        .spawn()
    {
        error!("Failed to spawn web-server: {}", e);
    }

    // create & start engine
    Engine::new(cache_provider, members).start(true);
}

use dotenv::dotenv;

use cache::{CacheProvider, MapProvider};
use engine::engine::Engine;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();

    let data: &str = "./data/pack";
    let members: bool = std::env::var("MEMBERS").unwrap() == "true";

    // io load cache
    let cache_provider: CacheProvider =
        CacheProvider::io(data, std::env::var("COMPILER_VERSION").unwrap(), members);

    // create engine
    let mut engine: Engine = Engine::new(cache_provider, members);

    // start engine
    engine.start(true, MapProvider::io(data));
}

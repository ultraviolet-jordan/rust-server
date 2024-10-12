use dotenv::dotenv;

use cache::ObjProvider;

fn main() {
    println!("Hello, world!");
    // ----
    dotenv().ok();
    //
    ObjProvider::io("./data/pack", std::env::var("MEMBERS").unwrap() == "true");
}

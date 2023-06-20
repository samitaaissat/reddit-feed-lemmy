mod lib;
use lib::config;
fn main() {
    let config = config::Config::load();
    println!("Hello, world!");
}

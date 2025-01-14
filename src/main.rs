use clap::Parser;
use img2ascii::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = img2ascii::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

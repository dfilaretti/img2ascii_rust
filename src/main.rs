use std::{env, process};
use img2ascii::Config;
use clap::Parser;

/// Simple program convert an image to ASCII art
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image file to convert
    #[arg(short, long)]
    file: String,

    /// Squeeze
    #[arg(short, long, default_value_t = 2)]
    squeeze: u8,
}

fn main() {
    // accept a filename as an argument (the only argument, actually)
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    if let Err(e) = img2ascii::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


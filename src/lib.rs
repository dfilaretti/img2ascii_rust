//! # img2ascii
//!
//! `img2ascii` is a collection of utilities to make performing certain
//! calculations more convenient.
//!
//! TODO: which methods to solve the "squashed ASCII art problem" to use?
//! - tweak the image size when resizing
//! - duplicate each font in the ASCII art
//!
//! Actually, why not implementing both and giving user a choice?

use clap::Parser;
use image::imageops::{resize, FilterType};
use std::error::Error;

/// Simple program convert an image to ASCII art
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Image file to convert
    #[arg(short, long)]
    input_file: String,

    /// Squeeze
    #[arg(short, long, default_value_t = 2)]
    squeeze: u8,
}

/// Convert a luminance value to a character
fn lumi_8_to_char(lumi: u8) -> char {
    match lumi {
        0..=63 => ' ',
        64..=127 => '.',
        128..=191 => 'o',
        192..=255 => '@',
    }
}

/// Downsample an image by a factor of `block_size`
/// TODO: this doesn't necessarily have to operate on `GreyImage` types; it could be more general (e.g. `DynamicImage`)
fn shrink_image(img: image::GrayImage, block_size: u32) -> image::GrayImage {
    let (width, height) = img.dimensions();
    let (new_width, new_height) = (width / block_size, height / block_size);
    let downsampled_img = resize(&img, new_width * 2, new_height, FilterType::Lanczos3);
    downsampled_img
}

/// Convert an image to ASCII art
fn img_to_ascii(img: image::GrayImage) -> String {
    img.enumerate_rows()
        // map each row to a string of ASCII characters (terminating with a newline)
        .map(|(_, row)| {
            row.map(|(_, _, lumi)| repeat_char(lumi_8_to_char(lumi.0[0]), 1))
                .collect::<String>()
                + "\n"
        })
        // collect all the rows into a single string
        .collect::<String>()
}

/// Repeat a character `c` `times` times
/// TODO: this certainly could be generic; also it's probably doable inline with built-in functions
pub fn repeat_char(c: char, times: usize) -> String {
    std::iter::repeat(c).take(times).collect()
}

/// Run the application
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // load image and convert to grey scale
    let img = image::open(config.input_file)?.into_luma8();
    println!("Original image dimensions: {:?}", img.dimensions());

    // get a smaller image (downsample)
    let block_size = 16;
    let img_smaller = shrink_image(img, block_size);

    println!("Reduced image size by a factor of {}x", block_size);
    println!(
        "Downsampled image dimensions: {:?}",
        img_smaller.dimensions()
    );

    // generate and print ASCII art
    let ascii_img = img_to_ascii(img_smaller);
    println!("{}", ascii_img);

    Ok(())
}
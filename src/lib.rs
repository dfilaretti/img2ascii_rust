//! # img2ascii
//!
//! Convert pictures into ASCII art, allowing to specify the width of the output (in characters).
//!

use clap::{Parser, ValueEnum};
use image::{
    imageops::{resize, FilterType},
    GenericImageView, ImageBuffer, Pixel,
};
use std::error::Error;

/// Simple program convert an image to ASCII art
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Image file to convert
    #[arg(short, long)]
    input_file: String,

    /// Expected width of the output ASCII art (in characters)
    #[arg(short, long, default_value_t = 80)]
    width_char: u16,

    /// Squeeze factor
    #[arg(short, long, default_value_t = 2)]
    squeeze: u8,

    /// Display some debug information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Correction mode
    #[arg(short, long, value_enum, default_value_t = CorrectionMode::Stretch)]
    correction_mode: CorrectionMode,
}

/// Mapping an (grey scale) image to ASCII pixel-by-pixel will usually lead to a distorted image, since 
/// the aspect ratio of the characters is not the same as the pixels (i.e. we can think of pixels as being 
/// perfect squares while characters are usually taller than they are wide). This can be corrected in different
/// way, and we offer two here. 
/// 
/// # RepeatChars
/// 
/// This is the simplest method, where we simply repeat each character 'n' times (horizontally). Since characters
/// are taller then wide, we can repeat each character 2 or 3 times to "build a square" (or close to it).
/// 
/// # Stretch
/// 
/// Here me map pixels to characters 1:1 (i.e. each pixel is represented by a single character) but in order to
/// fix the aspect ratio we stretch the image horizontally by a factor of 2 or 3 (before generating the ASCII art).
#[derive(ValueEnum, Debug, Clone)]
enum CorrectionMode {
    Stretch,
    RepeatChars,
}

/// Convert a luminance value to a character
/// NOTE: at the moment this is simply mapping the entire range of luminance values to a 4 different characters.
///       This could be improved by using a more sophisticated mapping, e.g. to use more characters.
fn lumi_8_to_char(lumi: u8) -> char {
    match lumi {
        0..=63 => ' ',
        64..=127 => '.',
        128..=191 => 'o',
        192..=255 => '@',
    }
}

fn shrink_image<T>(
    img: &T,
    config: &Config,
) -> ImageBuffer<T::Pixel, Vec<<T::Pixel as Pixel>::Subpixel>>
where
    T: GenericImageView,
    <T as GenericImageView>::Pixel: 'static,
{
    // TODO: check the number types here. Can we avoid all this conversions?
    let ratio = img.dimensions().0 as f32 / (config.width_char / config.squeeze as u16) as f32;
    let new_width = (img.dimensions().0 as f32 / ratio) as u32;
    let new_height = (img.dimensions().1 as f32 / ratio) as u32;

    resize(
        img,
        new_width * config.squeeze as u32,
        new_height,
        FilterType::Lanczos3,
    )
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
/// TODO: currently not using this. How about allowing users to chose whether to use this or the image stretch method instead?
pub fn repeat_char(c: char, times: usize) -> String {
    std::iter::repeat(c).take(times).collect()
}

/// Run the application
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // load image and convert to grey scale
    let img = image::open(&config.input_file)?.into_luma8();

    if config.verbose {
        println!("Loaded image: {}", config.input_file);
        println!("Original image dimensions: {:?}", img.dimensions());
    }

    // get a smaller image (downsample)
    let img_smaller = shrink_image(&img, &config);

    //println!("Reduced image size by a factor of {}x", block_size);
    if config.verbose {
        println!(
            "Downsampled image dimensions: {:?}",
            img_smaller.dimensions()
        );

        println!("Converting image to ASCII art...");
    }

    // generate and print ASCII art
    println!("{}", img_to_ascii(img_smaller));

    Ok(())
}

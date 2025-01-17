//! # img2ascii
//!
//! Convert images into ASCII art.
//!
//! User can provide additional options to specify the desired width in characters,
//! as well as the amount and type of horizontal correction to apply. (this is needed
//! because, unlike pixels which can be considered perfect squares, characters are more
//! tall than they are wide, meaning that directly mapping each pixel to a character will
//! result in a horizontally squeezed image).
//! 
//! # Future work 
//! 
//! - improve the mapping between luminance values and characters. 
//!   At the moment we map then entire range to only 4 characters 
//!   (see the `lumi_8_to_char`function) which actually looks surptisingly nice.  But can 
//!   we do any better?
//!

use clap::{Parser, ValueEnum};
use image::{
    imageops::{resize, FilterType},
    GenericImageView, ImageBuffer, Pixel,
};
use std::error::Error;

/// Convert an image file to ASCII art
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Image file to convert
    #[arg(short, long)]
    input: String,

    /// Desired width of the ASCII art (in characters)
    #[arg(short, long, default_value_t = 80)]
    width: u16,

    /// Horizontal adjustment mode
    #[arg(short, long, value_enum, default_value_t = HorizontalAdjustmentMode::Stretch)]
    mode: HorizontalAdjustmentMode,

    /// Horizontal adjustment amount
    #[arg(short, long, default_value_t = 2)]
    amount: u8,

    /// Display debug information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
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
enum HorizontalAdjustmentMode {
    /// Horizontally stretch the image before converting to ASCII
    Stretch,
    /// Repeat each character horizontally more than once
    Repeat,
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
    let ratio = img.dimensions().0 as f32 / (config.width / config.amount as u16) as f32;
    let new_width = (img.dimensions().0 as f32 / ratio) as u32;
    let new_height = (img.dimensions().1 as f32 / ratio) as u32;

    let width_mult_factor = match config.mode {
        HorizontalAdjustmentMode::Stretch => config.amount,
        _ => 1,
    };

    resize(
        img,
        new_width * width_mult_factor as u32,
        new_height,
        FilterType::Lanczos3,
    )
}

/// Convert an image to ASCII art
fn img_to_ascii(img: image::GrayImage, config: &Config) -> String {
    // repeat each char n times horizontally to fix aspect ratio issue
    let horiz_repeat_count = match config.mode {
        HorizontalAdjustmentMode::Repeat => config.amount,
        _ => 1,
    } as usize;

    img.enumerate_rows()
        // map each row to a string of ASCII characters (terminating with a newline)
        .map(|(_, row)| {
            row.map(|(_, _, lumi)| repeat_char(lumi_8_to_char(lumi.0[0]), horiz_repeat_count))
                .collect::<String>()
                + "\n"
        })
        // collect all the rows into a single string
        .collect::<String>()
}

/// Repeat a character `c` times
fn repeat_char(c: char, times: usize) -> String {
    std::iter::repeat(c).take(times).collect()
}

/// Run the application
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // load image and convert to grey scale
    let img = image::open(&config.input)?.into_luma8();

    if config.verbose {
        println!("Loaded image: {}", config.input);
        println!("Original image dimensions: {:?}", img.dimensions());
        println!("Desired ASCII art width (in characters): {}", config.width);
        println!("Aspect ratio correction method: {:?}", config.mode)
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
    println!("{}", img_to_ascii(img_smaller, &config));

    Ok(())
}

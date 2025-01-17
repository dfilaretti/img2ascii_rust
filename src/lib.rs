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
use std::{error::Error, iter::repeat_n};

/// Convert an image file to ASCII art
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Image file to convert
    #[arg(short, long)]
    input: String,

    /// Desired width of the ASCII art (in characters)
    #[arg(short, long, default_value_t = 80)]
    width: u32,

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
/// This is the simplest method, where we simply repeat each character 'n' times (horizontally) when generating
/// the ASCII art.
///
/// # Stretch
///
/// Here me map pixels to characters 1:1 (i.e. each pixel is represented by a single character) but in order to
/// fix the aspect ratio we stretch the input image horizontally by a factor of 2 or 3 to compensate for the
/// horizontal squeezing effect.
#[derive(ValueEnum, Debug, Clone)]
enum HorizontalAdjustmentMode {
    /// Horizontally stretch the image before converting to ASCII
    Stretch,
    /// Repeat each character horizontally more than once
    Repeat,
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

/// Build a downsampled (usually smaller) image based on the desired output width in characters
/// as well as on the chosen horizontal adjustment method.
fn downsample_image<P: Pixel + 'static>(
    img: &impl GenericImageView<Pixel = P>,
    config: &Config,
) -> ImageBuffer<P, Vec<P::Subpixel>>
where
{
    // TODO: overflow check
    let div_round = |n, d| (n + d / 2) / d;
    let amount = config.amount as u32;
    let new_width = div_round(config.width, amount);
    let new_height = div_round(
        config.width * img.dimensions().1,
        img.dimensions().0 * amount,
    );

    let width_mult_factor = match config.mode {
        HorizontalAdjustmentMode::Stretch => config.amount,
        HorizontalAdjustmentMode::Repeat => 1,
    };

    resize(
        img,
        new_width * width_mult_factor as u32,
        new_height,
        FilterType::Lanczos3,
    )
}

/// Convert a grey-scale image to ASCII art by mapping each pixel (consisting of
/// a single luminance value) to a character.
fn img_to_ascii(img: &image::GrayImage, config: &Config) -> String {
    // repeat each char n times horizontally to fix aspect ratio issue
    let horiz_repeat_count = match config.mode {
        HorizontalAdjustmentMode::Stretch => 1,
        HorizontalAdjustmentMode::Repeat => config.amount,
    } as usize;

    img.rows()
        // map each row to a string of ASCII characters (terminating with a newline)
        .flat_map(|row| {
            row.flat_map(|lumi| repeat_n(lumi_8_to_char(lumi.0[0]), horiz_repeat_count))
                .chain(std::iter::once('\n'))
        })
        // collect all the rows into a single string
        .collect::<String>()
}

/// Run the application
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // load image and convert to grey scale
    let img = image::open(&config.input)?.into_luma8();

    // print some debug information
    if config.verbose {
        println!("Loaded image: {}", config.input);
        println!("Original image dimensions: {:?}", img.dimensions());
        println!("Desired ASCII art width (in characters): {}", config.width);
        println!("Aspect ratio correction method: {:?}", config.mode)
    }

    // build the downsampled image
    let downsampled_image = downsample_image(&img, &config);

    // some more debug output if needed
    if config.verbose {
        println!(
            "Downsampled image dimensions: {:?}",
            downsampled_image.dimensions()
        );

        println!("Converting image to ASCII art...");
    }

    // generate and print ASCII art
    println!("{}", img_to_ascii(&downsampled_image, &config));

    Ok(())
}

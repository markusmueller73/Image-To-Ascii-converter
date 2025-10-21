// Image To Ascii converter for the console
//
// Copyright (c) 2025 Markus Müller
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod ascii_image;
mod config;
mod dithering;
mod greyscaling;
mod resizing;

use crate::ascii_image::*;
use crate::config::*;
use crate::dithering::*;
use crate::greyscaling::*;
use crate::resizing::*;
use image::{DynamicImage, ExtendedColorType, ImageFormat, ImageReader};
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), i32> {
    // get arguments
    let cfg = Configuration::parse();

    // load original image
    let original_image = load_image(&cfg.filename).unwrap();

    // calc the sizes for ascii and braille versions
    let (mut width, mut height) = calc_image_size(
        original_image.width(),
        original_image.height(),
        cfg.ascii_width,
        cfg.ascii_height,
    );

    // print conversion settings to console
    cfg.print(width, height);
    println!(
        "Image {} loaded successfully (size: {}x{}).",
        cfg.filename,
        original_image.width(),
        original_image.height()
    );

    // use different size for braille image
    if cfg.ascii_type == AsciiType::Braille {
        width *= ASCII_X_DOTS as u32;
        height *= ASCII_Y_DOTS as u32;
    }

    let scaled_vec = create_resized_image(&original_image, width, height, cfg.resize_opt);
    save_image("scaled.png", &scaled_vec, width, height);

    let grey_vec = create_greyscale_image(
        &scaled_vec,
        width,
        height,
        cfg.grey_scale,
        cfg.invert,
        cfg.alpha_threshold,
    );
    save_image("grey.png", &grey_vec, width, height);

    // the vector has more than 1 channel, we convert it to only 1 channel
    let asc_vec = if cfg.ascii_type == AsciiType::Dot || cfg.ascii_type == AsciiType::Braille {
        let dither_vec = create_dither_image(&grey_vec, width, height, cfg.threshold, cfg.dither);
        save_image("dither.png", &dither_vec, width, height);
        create_single_channel_vec(&dither_vec, width, height)
    } else {
        create_single_channel_vec(&grey_vec, width, height)
    };

    let asc_image: String = if cfg.ascii_type == AsciiType::Braille {
        ascii_type_braille(&asc_vec, width, height)
    } else {
        create_ascii_image(&asc_vec, cfg.ascii_type, width)
    };

    if cfg.show_ascii {
        println!("\n{asc_image}");
    }

    let file = match File::create(&cfg.output) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(5);
        }
    };

    let mut writer = BufWriter::new(file);
    writer
        .write_all(&asc_image.into_bytes())
        .unwrap_or_else(|err| {
            println!("Error: {}", err);
        });
    writer.flush().unwrap_or_else(|err| {
        println!("Error: {}", err);
    });

    println!("Ascii image {} successfully written.", cfg.output);

    Ok(())
}

fn load_image(filename: &str) -> std::io::Result<DynamicImage> {
    let new_image = ImageReader::open(filename)
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        })
        .with_guessed_format()
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(2);
        })
        .decode()
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(3);
        });
    Ok(new_image)
}

/// ugly coded, only for debug purposes
fn save_image(filename: &str, buffer: &[u8], width: u32, height: u32) {
    #[cfg(debug_assertions)]
    image::save_buffer_with_format(
        filename,
        buffer,
        width,
        height,
        ExtendedColorType::Rgba8,
        ImageFormat::Png,
    )
    .unwrap();
}

fn calc_image_size(
    img_width: u32,
    img_height: u32,
    ascii_width: u16,
    ascii_height: u16,
) -> (u32, u32) {
    let ar: f32 = if img_width > img_height {
        img_height as f32 / img_width as f32
    } else {
        img_width as f32 / img_height as f32
    };

    let cols: u32;
    let rows: u32;

    if ascii_width == 0 && ascii_height == 0 {
        cols = 80;
        rows = (cols as f32 * ASCII_X_DOTS as f32 * ar / ASCII_Y_DOTS as f32).ceil() as u32;
    } else if ascii_width == 0 && ascii_height > 0 {
        rows = ascii_height as u32;
        cols = (rows as f32 * ASCII_Y_DOTS as f32 * ar / ASCII_X_DOTS as f32).ceil() as u32;
    } else if ascii_width > 0 && ascii_height == 0 {
        cols = ascii_width as u32;
        rows = (cols as f32 * ASCII_X_DOTS as f32 * ar / ASCII_Y_DOTS as f32).ceil() as u32;
    } else {
        cols = ascii_width as u32;
        rows = ascii_height as u32;
    }

    (cols, rows)
}

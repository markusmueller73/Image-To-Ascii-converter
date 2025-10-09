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
//use image::{DynamicImage, ExtendedColorType, ImageFormat, ImageReader, RgbaImage};
use image::{DynamicImage, ImageReader, RgbaImage};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::exit;

fn main() -> Result<(), i32> {
    let cfg = Configuration::parse();

    let original_image = load_image(&cfg.filename).unwrap();
    println!("Image {} loaded successfully.", cfg.filename);

    let (mut width, mut height, b_width, b_height) = calc_image_size(
        &original_image,
        cfg.ascii_width,
        cfg.ascii_height,
    );
    // print conversion settings to console
    cfg.print(width, height);

    if cfg.ascii_type == AsciiType::Braille {
        width = b_width;
        height = b_height;
    }

    let scaled_vec = match cfg.resize_opt {
        ResizeAlgo::Bicubic => scaling_bicubic(&original_image.to_rgba8(), width, height),
        ResizeAlgo::Bilinear => scaling_bilinear(&original_image.to_rgba8(), width, height),
        ResizeAlgo::NearestNeighbour => {
            scaling_nearest_neighbour(&original_image.to_rgba8(), width, height)
        }
    };

    //image::save_buffer_with_format("scaled_image.png", &scaled_vec, width, height, ExtendedColorType::Rgba8, ImageFormat::Png).unwrap();
    let scaled_image = RgbaImage::from_raw(width, height, scaled_vec).unwrap();

    let mut grey_vec = create_greyscale_image(&scaled_image, cfg.grey_scale, cfg.invert, cfg.alpha_threshold);
    //image::save_buffer_with_format("grey_image.png", &grey_vec, width, height, ExtendedColorType::Rgb8, ImageFormat::Png).unwrap();

    if cfg.ascii_type == AsciiType::Dot || cfg.ascii_type == AsciiType::Braille {
        dither_image(&mut grey_vec, width, height, cfg.threshold, cfg.dither);
        //image::save_buffer_with_format("dither_image.png", &grey_vec, width, height, ExtendedColorType::Rgb8, ImageFormat::Png).unwrap();
    }

    // the vector has already 3 channels, we convert it to only 1 channel
    let asc_vec = convert_rgb_to_g_vec(&grey_vec);

    let asc_image: String;
    if cfg.ascii_type == AsciiType::Braille {
        asc_image = ascii_type_braille(&asc_vec, b_width, b_height);
    } else {
        asc_image = create_ascii_image(&asc_vec, cfg.ascii_type, width);
    }

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
    writer.write(&asc_image.into_bytes()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        0
    });
    writer.flush().unwrap_or_else(|err| {
        println!("Error: {}", err);
        return;
    });

    println!("Ascii image {} successfully written.", cfg.output);

    Ok(())
}

fn load_image(filename: &str) -> Result<DynamicImage,i32> {
    let new_image = ImageReader::open(filename)
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            exit(1);
        })
        .with_guessed_format()
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            exit(2);
        })
        .decode()
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            exit(3);
        });
    Ok(new_image)
}

fn calc_image_size(image: &DynamicImage, ascii_width: u16, ascii_height: u16) -> (u32, u32, u32, u32) {

    let img_width = image.width();
    let img_height = image.height();

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
    }  else if ascii_width > 0 && ascii_height == 0 {
        cols = ascii_width as u32;
        rows = (cols as f32 * ASCII_X_DOTS as f32 * ar / ASCII_Y_DOTS as f32).ceil() as u32;
    } else {
        cols = ascii_width as u32;
        rows = ascii_height as u32;
    }

    let braille_cols = cols * ASCII_X_DOTS as u32;
    let braille_rows = rows * ASCII_Y_DOTS as u32;

    (cols,rows,braille_cols,braille_rows)
}

fn convert_rgb_to_g_vec(rgb_vec: &Vec<u8>) -> Vec<u8> {
    let grey_len = rgb_vec.len() / 3;
    let mut grey_vec: Vec<u8> = Vec::with_capacity(grey_len);
    for r in rgb_vec.iter().step_by(3) {
        grey_vec.push(*r);
    }
    grey_vec
}
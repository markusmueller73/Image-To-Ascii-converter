use std::cmp::{max, min};
use image::{RgbaImage,Pixel};

#[derive(Debug)]
pub enum GreyScale {
    Average,
    Desaturate,
    Luminance,
    Maximum,
}

fn greyscale_average(red: u8, green: u8, blue: u8) -> u8 {
    (red + green + blue) / 3
}

fn greyscale_desaturate(red: u8, green: u8, blue: u8) -> u8 {
    let grey_max = max(max(red, green), blue);
    let grey_min = min(min(red, green), blue);
    (grey_max + grey_min) / 2
}

fn greyscale_luminance(red: u8, green: u8, blue: u8) -> u8 {
    (red as f32 * 0.2989 + green as f32 * 0.587 + blue as f32 * 0.114).round() as u8
}

fn greyscale_maximum(red: u8, green: u8, blue: u8) -> u8 {
    max(max(red, green), blue)
}

// pub fn convert_to_greyscale_1d(image: &RgbaImage, greyscale: GreyScale, invert: bool, alpha_threshold: u8) -> Vec<u8> {
// 
//     let (width,height) = image.dimensions();
//     let mut img_vec: Vec<u8> = Vec::with_capacity((width * height) as usize);
// 
//     for y in 0..height {
//         for x in 0..width {
// 
//             let pxl = image.get_pixel(x, y);
//             let rgba = pxl.channels();
// 
//             let grey: u8;
//             if rgba[3] < alpha_threshold {
//                 grey = 0;
//             } else {
//                 match greyscale {
//                     GreyScale::Average => grey = greyscale_average(rgba[0], rgba[1], rgba[2]),
//                     GreyScale::Desaturate => grey = greyscale_desaturate(rgba[0], rgba[1], rgba[2]),
//                     GreyScale::Luminance => grey = greyscale_luminance(rgba[0], rgba[1], rgba[2]),
//                     GreyScale::Maximum => grey = greyscale_maximum(rgba[0], rgba[1], rgba[2]),
//                 }
//             }
// 
//             if invert {
//                 img_vec.push(255 - grey);
//             } else {
//                 img_vec.push(grey);
//             }
//         }
//     }
// 
//     img_vec
// }

pub fn create_greyscale_image(image: &RgbaImage, greyscale: GreyScale, invert: bool, alpha_threshold: u8) -> Vec<u8> {

    let (width,height) = image.dimensions();
    let mut img_vec: Vec<u8> = Vec::with_capacity((width * height) as usize * 3);

    for y in 0..height {
        for x in 0..width {

            let pxl = image.get_pixel(x, y);
            let rgba = pxl.channels();

            let grey: u8;
            if rgba[3] < alpha_threshold {
                grey = 0;
            } else {
                match greyscale {
                    GreyScale::Average => grey = greyscale_average(rgba[0], rgba[1], rgba[2]),
                    GreyScale::Desaturate => grey = greyscale_desaturate(rgba[0], rgba[1], rgba[2]),
                    GreyScale::Luminance => grey = greyscale_luminance(rgba[0], rgba[1], rgba[2]),
                    GreyScale::Maximum => grey = greyscale_maximum(rgba[0], rgba[1], rgba[2]),
                }
            }

            if invert {
                img_vec.push(255 - grey);
                img_vec.push(255 - grey);
                img_vec.push(255 - grey);
            } else {
                img_vec.push(grey);
                img_vec.push(grey);
                img_vec.push(grey);
            }
        }
    }

    img_vec
}
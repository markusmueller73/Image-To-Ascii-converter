use std::cmp::{max, min};

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

pub fn create_greyscale_image(
    img_vec: &[u8],
    width: u32,
    height: u32,
    greyscale: GreyScale,
    invert: bool,
    alpha_threshold: u8,
) -> Vec<u8> {
    // calc channels from vector size
    let channels = img_vec.len() / (width * height) as usize;

    let mut grey_vec: Vec<u8> = Vec::with_capacity((width * height) as usize * channels);

    for y in 0..height {
        for x in 0..width {
            // calc where in the vector the correct pixel is
            let offset = channels * (y * width + x) as usize;
            let mut pxl: Vec<u8> = Vec::with_capacity(channels);
            for c in 0..channels {
                pxl.push(img_vec[offset + c]);
            }

            let grey: u8;
            if channels == 4 && pxl[3] < alpha_threshold {
                grey = 0;
            } else {
                match greyscale {
                    GreyScale::Average => grey = greyscale_average(pxl[0], pxl[1], pxl[2]),
                    GreyScale::Desaturate => grey = greyscale_desaturate(pxl[0], pxl[1], pxl[2]),
                    GreyScale::Luminance => grey = greyscale_luminance(pxl[0], pxl[1], pxl[2]),
                    GreyScale::Maximum => grey = greyscale_maximum(pxl[0], pxl[1], pxl[2]),
                }
            }

            for _ in pxl {
                if invert {
                    grey_vec.push(255 - grey);
                } else {
                    grey_vec.push(grey);
                }
            }
        }
    }

    grey_vec
}

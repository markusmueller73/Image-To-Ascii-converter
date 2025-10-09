use std::cmp::min;
use image::{Pixel, RgbaImage};

#[derive(Debug)]
pub enum ResizeAlgo {
    Bicubic,
    Bilinear,
    NearestNeighbour,
}

pub fn scaling_nearest_neighbour(rgba_img: &RgbaImage, new_width: u32, new_height: u32) -> Vec<u8> {
    let (img_width, img_height) = rgba_img.dimensions();
    let channels = rgba_img.get_pixel(0, 0).channels().len();

    let mut img_vec: Vec<u8> = Vec::with_capacity((new_width * new_height) as usize * channels);

    for y in 0..new_height {
        for x in 0..new_width {
            let mut org_x = (x as f32 / new_width as f32 * img_width as f32).round() as u32;
            let mut org_y = (y as f32 / new_height as f32 * img_height as f32).round() as u32;

            org_x = min(org_x, img_width);
            org_y = min(org_y, img_height);

            let rgba = rgba_img.get_pixel(org_x, org_y).channels();
            img_vec.push(rgba[0]);
            img_vec.push(rgba[1]);
            img_vec.push(rgba[2]);
            img_vec.push(rgba[3]);
        }
    }
    img_vec
}

pub fn scaling_bilinear(rgba_img: &RgbaImage, new_width: u32, new_height: u32) -> Vec<u8> {
    let (img_width, img_height) = rgba_img.dimensions();
    let channels = rgba_img.get_pixel(0, 0).channels().len();

    let mut img_vec: Vec<u8> = Vec::with_capacity((new_width * new_height) as usize * channels);

    let scale_x: f32 = img_width as f32 / new_width as f32;
    let scale_y: f32 = img_height as f32 / new_height as f32;

    for y in 0..new_height {
        for x in 0..new_width {
            let x0 = (scale_x * x as f32).floor() as u32;
            let y0 = (scale_y * y as f32).floor() as u32;
            let x1 = (scale_x * x as f32).ceil() as u32;
            let y1 = (scale_y * y as f32).ceil() as u32;

            let x_weight = (scale_x * x as f32) - x0 as f32;
            let y_weight = (scale_y * y as f32) - y0 as f32;

            let color_x0y0 = rgba_img.get_pixel(x0, y0).channels();
            let color_x1y0 = rgba_img.get_pixel(x1, y0).channels();
            let color_x0y1 = rgba_img.get_pixel(x0, y1).channels();
            let color_x1y1 = rgba_img.get_pixel(x1, y1).channels();

            for c in 0..channels {
                let interpolated_color = color_x0y0[c] as f32 * (1. - x_weight) * (1. - y_weight)
                    + color_x1y0[c] as f32 * x_weight * (1. - y_weight)
                    + color_x0y1[c] as f32 * (1. - x_weight) * y_weight
                    + color_x1y1[c] as f32 * x_weight * y_weight;

                img_vec.push(interpolated_color.round() as u8);
            }
        }
    }

    img_vec
}

fn clamp_f32(value: f32, min_val: f32, max_val: f32) -> f32 {
    if value < min_val {
        min_val
    } else if value > max_val {
        max_val
    } else {
        value
    }
}

fn get_pixel_clamped(rgba_img: &RgbaImage, x: i32, y: i32) -> &[u8] {
    let px: u32;
    if x < 0 {
        px = 0;
    } else if x >= rgba_img.width() as i32 {
        px = rgba_img.width() - 1;
    } else {
        px = x as u32;
    }
    let py: u32;
    if y < 0 {
        py = 0;
    } else if y >= rgba_img.height() as i32 {
        py = rgba_img.height() - 1;
    } else {
        py = y as u32;
    }
    rgba_img.get_pixel(px, py).channels()
}

fn cubic_hermite(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    let a = -p0 / 2. + (3. * p1) / 2. - (3. * p2) / 2. + p3 / 2.;
    let b = p0 - (5. * p1) / 2. + 2. * p2 - p3 / 2.;
    let c = -p0 / 2. + p2 / 2.;
    let d = p1;
    a * t * t * t + b * t * t + c * t + d
}

pub fn scaling_bicubic(rgba_img: &RgbaImage, new_width: u32, new_height: u32) -> Vec<u8> {
    let (img_width, img_height) = rgba_img.dimensions();
    let channels = rgba_img.get_pixel(0, 0).channels().len();

    let mut img_vec: Vec<u8> = Vec::with_capacity((new_width * new_height) as usize * channels);

    for y in 0..new_height {
        let py = y as f32 / new_height as f32;

        let y0 = py * img_height as f32 - 0.5;
        let y_pos = y0.floor() as i32;
        let y_fract = y0 - y0.floor();

        for x in 0..new_width {
            let px = x as f32 / new_width as f32;

            let x0 = px * img_width as f32 - 0.5;
            let x_pos = x0.floor() as i32;
            let x_fract = x0 - x0.floor();

            // 1st row
            let p00 = get_pixel_clamped(rgba_img, x_pos - 1, y_pos - 1);
            let p10 = get_pixel_clamped(rgba_img, x_pos + 0, y_pos - 1);
            let p20 = get_pixel_clamped(rgba_img, x_pos + 1, y_pos - 1);
            let p30 = get_pixel_clamped(rgba_img, x_pos + 2, y_pos - 1);

            // 2nd row
            let p01 = get_pixel_clamped(rgba_img, x_pos - 1, y_pos + 0);
            let p11 = get_pixel_clamped(rgba_img, x_pos + 0, y_pos + 0);
            let p21 = get_pixel_clamped(rgba_img, x_pos + 1, y_pos + 0);
            let p31 = get_pixel_clamped(rgba_img, x_pos + 2, y_pos + 0);

            // 3rd row
            let p02 = get_pixel_clamped(rgba_img, x_pos - 1, y_pos + 1);
            let p12 = get_pixel_clamped(rgba_img, x_pos + 0, y_pos + 1);
            let p22 = get_pixel_clamped(rgba_img, x_pos + 1, y_pos + 1);
            let p32 = get_pixel_clamped(rgba_img, x_pos + 2, y_pos + 1);

            // 4th row
            let p03 = get_pixel_clamped(rgba_img, x_pos - 1, y_pos + 2);
            let p13 = get_pixel_clamped(rgba_img, x_pos + 0, y_pos + 2);
            let p23 = get_pixel_clamped(rgba_img, x_pos + 1, y_pos + 2);
            let p33 = get_pixel_clamped(rgba_img, x_pos + 2, y_pos + 2);

            for c in 0..channels {
                let col0 = cubic_hermite(
                    p00[c] as f32,
                    p10[c] as f32,
                    p20[c] as f32,
                    p30[c] as f32,
                    x_fract,
                );
                let col1 = cubic_hermite(
                    p01[c] as f32,
                    p11[c] as f32,
                    p21[c] as f32,
                    p31[c] as f32,
                    x_fract,
                );
                let col2 = cubic_hermite(
                    p02[c] as f32,
                    p12[c] as f32,
                    p22[c] as f32,
                    p32[c] as f32,
                    x_fract,
                );
                let col3 = cubic_hermite(
                    p03[c] as f32,
                    p13[c] as f32,
                    p23[c] as f32,
                    p33[c] as f32,
                    x_fract,
                );

                let value = cubic_hermite(col0, col1, col2, col3, y_fract);

                img_vec.push(clamp_f32(value, 0., 255.) as u8);
            }
        }
    }

    img_vec
}

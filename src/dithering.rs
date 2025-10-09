#[derive(Debug, PartialEq)]
pub enum Dithering {
    Atkinson,
    Burkes,
    FloydSteinberg,
    JJN,
    NoDithering,
    Sierra,
    SierraLite,
    Stucki,
    TwoRowSierra,
}

const DITHER_ATKINSON_MATRIX: [[i32; 3]; 6] = [[1,0,1],[2,0,1],[-1,1,1],[0,1,1],[1,1,1],[0,2,1]];
const DITHER_ATKINSON_DIVISOR: f32 = 8.;
const DITHER_FLOYDSTEINBERG_MATRIX: [[i32; 3]; 4] = [[1,0,7],[-1,1,3],[0,1,5],[1,1,1]];
const DITHER_FLOYDSTEINBERG_DIVISOR: f32 = 16.;
const DITHER_JJN_MATRIX: [[i32; 3]; 12] = [[1,0,7],[2,0,5],[-2,1,3],[-1,1,5],[0,1,7],[1,1,5],[2,1,3],[-2,2,1],[-1,2,3],[0,2,5],[1,2,3],[2,2,1]];
const DITHER_JJN_DIVISOR: f32 = 48.;
const DITHER_STUCKI_MATRIX: [[i32; 3]; 12] = [[1,0,8],[2,0,4],[-2,1,2],[-1,1,4],[0,1,8],[1,1,4],[2,1,2],[-2,2,1],[-1,2,2],[0,2,4],[1,2,2],[2,2,1]];
const DITHER_STUCKI_DIVISOR: f32 = 42.;
const DITHER_BURKES_MATRIX: [[i32; 3]; 7] = [[1,0,8],[2,0,4],[-2,1,2],[-1,1,4],[0,1,8],[1,1,4],[2,1,2]];
const DITHER_BURKES_DIVISOR: f32 = 32.;
const DITHER_SIERRA_MATRIX: [[i32; 3]; 10] = [[1,0,5],[2,0,3],[-2,1,2],[-1,1,4],[0,1,5],[1,1,4],[2,1,2],[-1,2,2],[0,2,3],[1,2,2]];
const DITHER_SIERRA_DIVISOR: f32 = 32.;
const DITHER_TWO_ROW_SIERRA_MATRIX: [[i32; 3]; 7] = [[1,0,4],[2,0,3],[-2,1,1],[-1,1,2],[0,1,3],[1,1,2],[2,1,1]];
const DITHER_TWO_ROW_SIERRA_DIVISOR: f32 = 16.;
const DITHER_SIERRA_LITE_MATRIX: [[i32; 3]; 3] = [[1,0,2],[-1,1,1],[0,1,1]];
const DITHER_SIERRA_LITE_DIVISOR: f32 = 4.;

fn is_in_bound(x: i32, y: i32, width: i32, height: i32) -> bool {
    if x >= 0 && x < width && y >= 0 && y < height {
        return true;
    }
    false
}

fn get_offset(x: u32, y: u32, width: u32) -> usize {
    (width * y + x) as usize * 3
}

fn clamp_dither_value(input: u8, diff: u8, factor: u8) -> u8 {
    let val = (input as f32 + diff as f32 * factor as f32).round() as i32;
    if val < 0 {
        0
    } else if val > 255 {
        255
    } else {
        val as u8
    }
}

pub fn dither_image(img_vec: &mut Vec<u8>, width: u32, height: u32, threshold: u8, dither_type: Dithering) {

    if dither_type == Dithering::NoDithering { return; }

    for y in 0..height {
        for x in 0..width {

            let offset = get_offset(x, y, width);
            let rgb: [u8; 3] = [img_vec[offset],img_vec[offset+1],img_vec[offset+2]];

            let grey_val: u8;
            let diff_val: u8;
            let divisor = match dither_type {
                Dithering::Atkinson => DITHER_ATKINSON_DIVISOR,
                Dithering::Burkes => DITHER_BURKES_DIVISOR,
                Dithering::FloydSteinberg => DITHER_FLOYDSTEINBERG_DIVISOR,
                Dithering::JJN => DITHER_JJN_DIVISOR,
                Dithering::Sierra => DITHER_SIERRA_DIVISOR,
                Dithering::SierraLite => DITHER_SIERRA_LITE_DIVISOR,
                Dithering::Stucki => DITHER_STUCKI_DIVISOR,
                Dithering::TwoRowSierra => DITHER_TWO_ROW_SIERRA_DIVISOR,
                _ => 1.,
            };

            if rgb[0] < threshold {
                grey_val = 0;
                diff_val = (rgb[0] as f32 / divisor).round() as u8;
            } else {
                grey_val = 255;
                diff_val = ((rgb[0] as f32 - 255.) / divisor).round() as u8;
            }
            img_vec[offset] = grey_val;
            img_vec[offset+1] = grey_val;
            img_vec[offset+2] = grey_val;

            let dither_vec = match dither_type {
                Dithering::Atkinson => Vec::from(DITHER_ATKINSON_MATRIX),
                Dithering::Burkes => Vec::from(DITHER_BURKES_MATRIX),
                Dithering::FloydSteinberg => Vec::from(DITHER_FLOYDSTEINBERG_MATRIX),
                Dithering::JJN => Vec::from(DITHER_JJN_MATRIX),
                Dithering::Sierra => Vec::from(DITHER_SIERRA_MATRIX),
                Dithering::SierraLite => Vec::from(DITHER_SIERRA_LITE_MATRIX),
                Dithering::Stucki => Vec::from(DITHER_STUCKI_MATRIX),
                Dithering::TwoRowSierra => Vec::from(DITHER_TWO_ROW_SIERRA_MATRIX),
                _ => Vec::new(),
            };

            for p in dither_vec.iter() {

                if is_in_bound(x as i32 + p[0], y as i32 + p[1], width as i32, height as i32) {
                    let offs = get_offset((x as i32 + p[0]) as u32, (y as i32 + p[1]) as u32, width);
                    img_vec[offs] = clamp_dither_value(img_vec[offs], diff_val, p[2] as u8);
                    img_vec[offs+1] = img_vec[offs];
                    img_vec[offs+2] = img_vec[offs];
                }
            }

        }
    }

}


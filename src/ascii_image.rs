const ASCII_CHARS_BLOCK: [char; 5] = ['█', '▓', '▒', '░', ' '];
const ASCII_CHARS_EXTENDED: [char; 70] = [
    '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w',
    'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j',
    'f', 't', '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>',
    'i', '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'', '.', ' ',
];
const ASCII_CHARS_SIMPLE: [char; 10] = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

const BRAILLE_SHIFT_VALUE: [u32; 8] = [0, 1, 2, 6, 3, 4, 5, 7];
pub const ASCII_X_DOTS: usize = 2;
pub const ASCII_Y_DOTS: usize = 4;

#[derive(Debug, PartialEq)]
pub enum AsciiType {
    Block,
    Braille,
    Dot,
    Extended,
    Simple,
}

fn ascii_type_block(grey_value: u8) -> char {
    let val = ((grey_value as f32 * 4.) / 255.).round() as usize;
    ASCII_CHARS_BLOCK[val]
}

fn get_vector_offset(x: u32, y: u32, width: u32) -> usize {
    (width * y + x) as usize
}

pub fn ascii_type_braille(img_vec: &[u8], width: u32, height: u32) -> String {
    let mut braille_text = String::new();

    for iy in (0..height).step_by(ASCII_Y_DOTS) {
        for ix in (0..width).step_by(ASCII_X_DOTS) {
            let mut braille_info: [u8; 8] = [0; 8];

            let mut info_counter: usize = 0;
            for y in 0..ASCII_Y_DOTS as u32 {
                for x in 0..ASCII_X_DOTS as u32 {
                    if img_vec[get_vector_offset(ix + x, iy + y, width)] == 0 {
                        braille_info[info_counter] = 1;
                    }

                    info_counter += 1;
                }
            }

            let mut braille = 0;
            for n in 0..8 {
                braille += braille_info[n] << BRAILLE_SHIFT_VALUE[n];
            }

            let uni_vec: Vec<u16> = vec![braille as u16 + 10240];
            let uni_char = char::decode_utf16(uni_vec)
                .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
                .collect::<String>();
            braille_text.push_str(uni_char.as_str());
        }
        braille_text.push('\n');
    }

    braille_text
}

fn ascii_type_dot(grey_value: u8) -> char {
    if grey_value < 128 { '.' } else { ' ' }
}

fn ascii_type_extended(grey_value: u8) -> char {
    let val = ((grey_value as f32 * 69.) / 255.).round() as usize;
    ASCII_CHARS_EXTENDED[val]
}

fn ascii_type_simple(grey_value: u8) -> char {
    let val = ((grey_value as f32 * 9.) / 255.).round() as usize;
    ASCII_CHARS_SIMPLE[val]
}

pub fn create_ascii_image(img_vec: &[u8], ascii_type: AsciiType, width: u32) -> String {
    let mut asc_image = String::new();
    let mut pos: u32 = 0;

    for grey_val in img_vec.iter() {
        let ch = match ascii_type {
            AsciiType::Block => ascii_type_block(*grey_val),
            AsciiType::Dot => ascii_type_dot(*grey_val),
            AsciiType::Extended => ascii_type_extended(*grey_val),
            _ => ascii_type_simple(*grey_val),
        };

        asc_image.push(ch);

        pos += 1;
        if pos == width {
            pos = 0;
            asc_image.push('\n');
        }
    }

    asc_image
}

pub fn create_single_channel_vec(img_vec: &[u8], width: u32, height: u32) -> Vec<u8> {
    let channels = img_vec.len() / (width * height) as usize;
    let mut s_vec: Vec<u8> = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let offset = (width * y + x) as usize * channels;
            s_vec.push(img_vec[offset]);
        }
    }
    s_vec
}

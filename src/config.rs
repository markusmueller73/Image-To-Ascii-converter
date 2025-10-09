use crate::ascii_image::AsciiType;
use crate::dithering::Dithering;
use crate::greyscaling::GreyScale;
use crate::resizing::ResizeAlgo;
use std::env;
use std::process::exit;

#[derive(Debug)]
pub struct Configuration {
    pub filename: String,
    pub output: String,
    pub threshold: u8,
    pub alpha_threshold: u8,
    pub invert: bool,
    pub dither: Dithering,
    pub grey_scale: GreyScale,
    pub ascii_type: AsciiType,
    pub ascii_width: u16,
    pub ascii_height: u16,
    pub resize_opt: ResizeAlgo,
    pub show_ascii: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            filename: String::new(),
            output: String::new(),
            threshold: 180,
            alpha_threshold: 30,
            invert: false,
            dither: Dithering::TwoRowSierra,
            grey_scale: GreyScale::Luminance,
            ascii_type: AsciiType::Simple,
            ascii_width: 80,
            ascii_height: 0,
            resize_opt: ResizeAlgo::Bilinear,
            show_ascii: false,
        }
    }
}

impl Configuration {
    pub fn parse() -> Configuration {
        let mut cfg = Configuration::default();

        let mut args = env::args();

        let prg_name = args.nth(0).unwrap_or_default();
        let prg_version = env!("CARGO_PKG_VERSION").to_string();

        if args.len() < 1 {
            println!("\nNot enough arguments, try: {} --help\n", prg_name);
            exit(99);
        }

        while let Some(arg) = args.next() {
            match arg[..].to_lowercase().as_str() {
                "-a" | "--ascii" => {
                    let next_arg = args.next().unwrap().to_uppercase();
                    match next_arg.as_str() {
                        "1" | "BLO" | "BLOCK" => cfg.ascii_type = AsciiType::Block,
                        "2" | "BRA" | "BRAILLE" => cfg.ascii_type = AsciiType::Braille,
                        "3" | "DOT" => cfg.ascii_type = AsciiType::Dot,
                        "4" | "EXT" | "EXTENDED" => cfg.ascii_type = AsciiType::Extended,
                        "5" | "SIM" | "SIMPLE" => cfg.ascii_type = AsciiType::Simple,
                        _ => println!("Unknown positional argument {} for ascii type.", next_arg),
                    }
                }

                "-at" | "--alpha-threshold" => {
                    let next_arg = args.next().unwrap_or("?".to_string());
                    cfg.alpha_threshold = next_arg.parse::<u8>().unwrap_or_else(|_| {
                        println!(
                            "Unknown positional argument {} for threshold, set to default (32).",
                            next_arg
                        );
                        32
                    });
                }

                "-d" | "--dither" | "--dithering" => {
                    let next_arg = args.next().unwrap().to_uppercase();
                    match next_arg.as_str() {
                        "0" | "NONE" | "NODITHERING" => cfg.dither = Dithering::NoDithering,
                        "1" | "ATK" | "ATKINSON" => cfg.dither = Dithering::Atkinson,
                        "2" | "BUR" | "BURKES" => cfg.dither = Dithering::Burkes,
                        "3" | "FLO" | "FLOYDSTEINBERG" => cfg.dither = Dithering::FloydSteinberg,
                        "4" | "JJN"  => cfg.dither = Dithering::FloydSteinberg,
                        "5" | "SIE" | "SIERRA" => cfg.dither = Dithering::FloydSteinberg,
                        "6" | "SIL" | "SIERRALITE" => cfg.dither = Dithering::FloydSteinberg,
                        "7" | "STU" | "STUCKI" => cfg.dither = Dithering::FloydSteinberg,
                        "8" | "TRS" | "TWOROWSIERRA" => cfg.dither = Dithering::FloydSteinberg,
                        _ => println!("Unknown positional argument {} for dithering.", next_arg),
                    }
                },

                "-f" | "--file" | "--filename" => {
                    let next_arg = args.next().unwrap();
                    cfg.filename = next_arg;
                }

                "-g" | "--grey" | "--greyscale" => {
                    let next_arg = args.next().unwrap().to_uppercase();
                    match next_arg.as_str() {
                        "1" | "AVG" | "AVERAGE" => cfg.grey_scale = GreyScale::Average,
                        "2" | "DES" | "DESATURATE" => cfg.grey_scale = GreyScale::Desaturate,
                        "3" | "LUM" | "LUMINANCE" => cfg.grey_scale = GreyScale::Luminance,
                        "4" | "MAX" | "MAXIMUM" => cfg.grey_scale = GreyScale::Maximum,
                        _ => println!("Unknown positional argument {} for greyscale.", next_arg),
                    }
                }

                "-h" | "--height" | "ascii_height" => {
                    let next_arg = args.next().unwrap_or("?".to_string());
                    cfg.ascii_height = next_arg.parse::<u16>().unwrap_or_else(|_| {
                        println!("Unknown positional argument {} for ascii height.", next_arg);
                        0
                    });
                }

                "--help" => {
                    Configuration::help(&prg_name);
                    exit(0);
                }

                "-i" | "--invert" => {
                    cfg.invert = true;
                }

                "-r" | "--resize" => {
                    let next_arg = args.next().unwrap().to_uppercase();
                    match next_arg.as_str() {
                        "1" | "BIC" | "BICUBIC" => cfg.resize_opt = ResizeAlgo::Bicubic,
                        "2" | "BIL" | "BILINEAR" => cfg.resize_opt = ResizeAlgo::Bilinear,
                        "3" | "NEA" | "NEAREST" => cfg.resize_opt = ResizeAlgo::NearestNeighbour,
                        _ => println!(
                            "Unknown positional argument {} for resize option.",
                            next_arg
                        ),
                    }
                }

                "-s" | "--show" => {
                    cfg.show_ascii = true;
                }

                "-t" | "--threshold" => {
                    let next_arg = args.next().unwrap_or("?".to_string());
                    cfg.threshold = next_arg.parse::<u8>().unwrap_or_else(|_| {
                        println!(
                            "Unknown positional argument {} for threshold, set to default (128).",
                            next_arg
                        );
                        128
                    });
                }

                "-V" | "--version" => {
                    println!("\n{} v{}\n", prg_name, prg_version);
                    exit(0);
                }

                "-w" | "--width" | "ascii_width" => {
                    let next_arg = args.next().unwrap_or("?".to_string());
                    cfg.ascii_width = next_arg.parse::<u16>().unwrap_or_else(|_| {
                        println!(
                            "Unknown positional argument {} for ascii width, set to default (60).",
                            next_arg
                        );
                        60
                    });
                }

                _ => {
                    if arg.starts_with('-') {
                        println!("Unknown argument {}", arg);
                    } else {
                        cfg.filename = arg;
                    }
                }
            }
        }

        if cfg.output.is_empty() {
            let mut split = cfg.filename.split('/');
            let fname = split.last().unwrap().to_string();
            split = fname.split('.');
            cfg.output = split.nth(0).unwrap().to_string();
            cfg.output.push_str(".txt");
        }

        cfg
    }

    pub fn print(&self, width: u32, height: u32) {
        println!("Creating ascii image with this settings:");
        println!("----------------------------------------");
        println!("Ascii size:\t{} cols x {} rows", width, height);
        println!("Ascii type:\t{}", Self::get_asc_type_desc(&self.ascii_type));
        println!("Dithering:\t{}", Self::get_dither_desc(&self.dither));
        println!("Greyscale:\t{}", Self::get_greyscale_desc(&self.grey_scale));
        println!("Scale filter:\t{}", Self::get_resize_desc(&self.resize_opt));
        println!("Invert colors:\t{}", self.invert);
        println!("Threshold:\t{}", self.threshold);
        println!();
    }

    fn help(name: &str) {
        println!("\n{} - Image to Ascii converter\n", name);
        println!("Usage:\n{} <FILE> [OPTIONS]\n", name);
        println!("Options:\n--------\n");
        println!("-a <TYPE>\t--ascii <TYPE>\t\ttype of ascii char set");
        println!("-f <FILE>\t--filename <FILE>\tpath and filename from the image file");
        println!("-g <TYPE>\t--greyscale <TYPE>\tthe greyscale conversion algorithm");
        println!("-h <NUM>\t--height <NUM>\t\tthe height of the ascii image");
        println!("  \t\t--help\t\t\tshow this help text");
        println!("-i\t\t--invert\t\tinvert the image colors");
        println!("-r <TYPE>\t--resize <TYPE>\t\tthe resize algorithm");
        println!("-t <NUM>\t--threshold <NUM>\tthe threshold from black (0) to white (255)");
        println!("-V\t\t--version\t\tthe version of {}", name);
        println!("-w <NUM>\t--width <NUM>\t\tthe width of the ascii image");
        println!();
        println!("Ascii types:\n------------");
        println!("| 1 | blo | ascii block chars (5 chars)");
        println!("| 2 | bra | braille chars");
        println!("| 3 | dot | only the . (dot)");
        println!("| 4 | ext | 70 chars");
        println!("| 5 | sim | 10 chars [default]");
        println!();
        println!("Dithering algorithms:\n---------------------");
        println!("| 0 | none  | no dithering");
        println!("| 1 | atk | Atkinson");
        println!("| 2 | bur | Burkes");
        println!("| 3 | flo | FloydSteinberg");
        println!("| 4 | jjn | Jarvis, Judike and Ninke");
        println!("| 5 | sie | Sierra");
        println!("| 6 | sil | Sierra Lite");
        println!("| 7 | stu | Stucki");
        println!("| 8 | trs | Two-Row Sierra [default]");
        println!();
        println!("Greyscale algorithms:\n---------------------");
        println!("| 1 | avg | Average");
        println!("| 2 | des | Desaturate");
        println!("| 3 | lum | Luminance [default]");
        println!("| 4 | max | Maximum");
        println!();
        println!("Resize algorithms:\n---------------------");
        println!("| 1 | bicu | Bi-Cubic (best)");
        println!("| 2 | bili | Bilinear [default]");
        println!("| 3 | near | Nearest Neighbour (fastest)");
        println!();
        println!("Hints:\n------");
        println!(
            "You can set width or height, the other size will be calculated by aspect ratio.\nIf you set both the ascii mage will be deformed to this size."
        );
        println!();
    }

    fn get_asc_type_desc(t: &AsciiType) -> &'static str {
        match t {
            AsciiType::Block => "Blocks",
            AsciiType::Braille => "Braille",
            AsciiType::Dot => "Dot",
            AsciiType::Extended => "Extended",
            AsciiType::Simple => "Simple",
        }
    }

    fn get_dither_desc(t: &Dithering) -> &'static str {
        match t {
            Dithering::Atkinson => "Atkinson",
            Dithering::Burkes => "Burkes",
            Dithering::FloydSteinberg => "Floyd and Steinberg",
            Dithering::JJN => "Jarvis, Judike and Ninke",
            Dithering::Sierra => "Sierra",
            Dithering::SierraLite => "Sierra Lite",
            Dithering::Stucki => "Stucki",
            Dithering::TwoRowSierra => "Two-Row Sierra",
            _ => "No dithering",
        }
    }

    fn get_greyscale_desc(t: &GreyScale) -> &'static str {
        match t {
            GreyScale::Average => "Average",
            GreyScale::Desaturate => "Desaturate",
            GreyScale::Luminance => "Luminance",
            GreyScale::Maximum => "Maximum",
        }
    }

    fn get_resize_desc(t: &ResizeAlgo) -> &'static str {
        match t {
            ResizeAlgo::Bicubic => "Bi-Cubic",
            ResizeAlgo::Bilinear => "Bilinear",
            ResizeAlgo::NearestNeighbour => "Nearest Neighbour",
        }
    }

    // fn split_at_equal_sign(arg: String) -> (String,String) {
    //     if arg.contains('=') {
    //         let str_vec: Vec<&str> = arg.split('=').collect();
    //         return (str_vec[0].to_string(),str_vec[1].to_string());
    //     }
    //     (String::new(),String::new())
    // }
}

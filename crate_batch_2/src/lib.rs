use flif::Flif;
use geo::{coord, ConvexHull, LineString};
use gif::DecodeOptions;
use goblin::archive::Index;
use handlebars::Handlebars;
use human_name::Name;
use hyper::{Uri, Url};
use image::ImageDecoder;
use rand::{seq::SliceRandom, thread_rng};
use serde_hjson::error::Result as HjsonResult;
use serde_hjson::{Map, Value};
use std::fs::{read_dir, File};
use std::io::{Cursor, Read};

// pub struct BenchmarkData {
//     pub run2_font: Vec<u8>,
//     pub run2_fontdue_data: Vec<u8>,
//     pub run3_coords: Vec<(f64, f64)>,
//     pub run4_buffer: Vec<u8>,
//     pub run5_template_error: String,
//     pub run5_template_block: String,
//     pub run6_data: Vec<u8>,
//     pub run7_name: String,
//     pub run8_url: String,
//     pub run9_bytes: Vec<u8>,
//     pub run10_gif_data: Vec<u8>,
//     pub run11_regex: String,
// }
//
// impl Default for BenchmarkData {
//     fn default() -> Self {
//         Self {
//             run2_font: include_bytes!("run2_font.bin").to_vec(),
//             run2_fontdue_data: vec![0u8; 100],
//             run3_coords: vec![
//                 (-10.0, 0.0),
//                 (5.0, -10.0),
//                 (10.0, 10.0),
//                 (-5.0, 10.0),
//             ],
//             run4_buffer: (&[]).to_vec(),
//             run5_template_error: "{{x[]@this}}".to_owned(),
//             run5_template_block: "{{#>(X)}}{{/X}}".to_owned(),
//             run6_data: vec![155],
//             run7_name: ".ΰ\u{330}\u{610}`".to_owned(),
//             run8_url: "http://test.com/nazghul?test=3".to_owned(),
//             run9_bytes: vec![0x52, 0x49, 0x46, 0x46, 0xaf, 0x50, 0x45, 0x33, 0x37, 0x44, 0x4d, 0x46],
//             run10_gif_data: (&[
//                 0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0xff, 0x00, 0xff,
//                 0xff, 0xff, 0x00, 0x00, 0x00, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c,
//                 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00,
//                 0x3b,
//             ]).to_vec(),
//             run11_regex: "\\u".to_owned(),
//         }
//     }
// }

pub struct BenchmarkData {
    pub testString: String,
    pub testString2: String,
    pub testVecU8: Vec<u8>,
    pub testU64: u64,
    pub testKey: [u8; 64],
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            testString: "Hello, Benchmark!".to_owned(),
            testString2: "Another test string".to_owned(),
            testVecU8: vec![1, 2, 3, 4, 5],
            testU64: 42,
            testKey: [0u8; 64],
        }
    }
}

pub fn main() {
    println!("crate batch 2 benchmark starting");
    let data = BenchmarkData::default();
    benchmark(&data);
}

pub fn benchmark(data: &BenchmarkData) {
    benchmark_artifacts(&data.testVecU8);
    benchmark_vec_u8(&data.testVecU8);
    benchmark_strings(&data.testString, &data.testString2); 
}

pub fn benchmark_artifacts(data: &[u8]) {
    // --- run 1 ---------------------------------------------------------------
    {
        // Interpret input bytes as a directory path (fallback to ".")
        let path_str = String::from_utf8_lossy(data).trim().to_string();
        let dir = if path_str.is_empty() { "." } else { &path_str };

        match read_dir(dir) {
            Ok(entries) => {
                for entry_res in entries {
                    let entry = match entry_res {
                        Ok(e) => e,
                        Err(e) => {
                            println!("Read dir entry error: {}", e);
                            continue;
                        }
                    };
                    let path = entry.path();
                    println!("Artifact: {}", path.display());

                    let mut contents = Vec::new();
                    match File::open(&path) {
                        Ok(mut file) => {
                            if let Err(e) = file.read_to_end(&mut contents) {
                                println!("Failed to read {}: {}", path.display(), e);
                                continue;
                            }
                            let _ = Flif::decode(Cursor::new(&contents))
                                .map(|img| img.get_raw_pixels());
                        }
                        Err(e) => {
                            println!("Failed to open {}: {}", path.display(), e);
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to read dir '{}': {}", dir, e);
            }
        }
    }
}


pub fn benchmark_vec_u8(data: &[u8]) {
    let mut order = vec![0, 1, 2, 3, 4, 5];
    let mut rng = thread_rng();
    order.shuffle(&mut rng);

    for idx in order {
        match idx {
            0 => {
                // --- run 2 ---------------------------------------------------------------
                {
                    let font_bytes = data.to_vec();
                    if let Ok(font) =
                        fontdue::Font::from_bytes(font_bytes, fontdue::FontSettings::default())
                    {
                        let (_metrics, _bitmap) = font.rasterize('g', 17.0);
                    }
                    println!("Hello, world!");

                    let second_font_bytes = data.to_vec();
                    if let Ok(font) = fontdue::Font::from_bytes(
                        second_font_bytes,
                        fontdue::FontSettings::default(),
                    ) {
                        println!("Font loaded successfully");
                    } else {
                        println!("Failed to load font");
                    }
                }
            }
            1 => {
                // --- run 3 ---------------------------------------------------------------
                {
                    let mut coords: Vec<_> = data
                        .chunks(2)
                        .map(|chunk| {
                            let x = chunk.get(0).copied().unwrap_or(0) as f64;
                            let y = chunk.get(1).copied().unwrap_or(0) as f64;
                            coord! { x: x, y: y }
                        })
                        .collect();

                    while coords.len() < 3 {
                        coords.push(coord! { x: 0.0, y: 0.0 });
                    }

                    let line_string = LineString(coords);
                    let res = line_string.convex_hull();
                    println!("Convex hull: {:?}", res);
                }
            }
            2 => {
                // --- run 4 ---------------------------------------------------------------
                {
                    if let Ok(index) = Index::parse_sysv_index(data) {
                        println!("Parsed index: {:?}", index);
                    } else {
                        println!("Failed to parse index");
                    }
                }
            }
            3 => {
                // --- run 6 ---------------------------------------------------------------
                {
                    let sample: HjsonResult<Map<String, Value>> = serde_hjson::from_slice(data);
                    println!("serde_hjson sample: {:?}", sample);
                }
            }
            4 => {
                // --- run 9 ---------------------------------------------------------------
                {
                    let cursor = Cursor::new(data.to_vec());
                    let _ = image::webp::WebPDecoder::new(cursor);
                }
            }
            5 => {
                // --- run 10 --------------------------------------------------------------
                {
                    let mut options = DecodeOptions::new();
                    options.set_color_output(gif::ColorOutput::RGBA);
                    let cursor = Cursor::new(data.to_vec());
                    if let Ok(mut decoder) = options.read_info(cursor) {
                        while let Ok(Some(_frame)) = decoder.read_next_frame() {}
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

pub fn benchmark_strings(str: &str, str2: &str) {
    let mut order = vec![0, 1, 2, 3];
    let mut rng = thread_rng();
    order.shuffle(&mut rng);

    for idx in order {
        match idx {
            0 => {
                // --- run 5 ---------------------------------------------------------------
                {
                    let context = Vec::<()>::new();
                    let hbs = Handlebars::new();
                    match hbs.render_template(str, &context) {
                        Ok(rendered) => println!("Handlebars rendered: {}", rendered),
                        Err(error) => println!("Handlebars error: {}", error),
                    }

                    let tpl = Handlebars::new();
                    let _ = tpl.render_template(str2, &Vec::<()>::new());
                }
            }
            1 => {
                // --- run 7 ---------------------------------------------------------------
                {
                    let name = Name::parse(str);
                    println!("Parsed name: {:?}", name);
                }
            }
            2 => {
                // --- run 8 ---------------------------------------------------------------
                {
                    let primary = str.to_owned();
                    let fallback = str2.to_owned();
                    let url = Url::parse(&primary)
                        .or_else(|_| Url::parse(&fallback))
                        .unwrap_or_else(|_| Url::parse("http://example.com").unwrap());
                    let _uri = Uri::from(url);
                }
            }
            3 => {
                // --- run 11 --------------------------------------------------------------
                {
                    let huffman_values = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
                    let _decoder = jpeg_decoder_63::Decoder::new(&huffman_values[..]);

                    let result =
                        image::codecs::jpeg::JpegDecoder::new(Cursor::new(huffman_values));
                    let decoder = match result {
                        Ok(d) => d,
                        Err(_) => return,
                    };
                    if decoder.total_bytes() > 2_000_000_000 {
                        return;
                    }
                    let mut buf = vec![0; decoder.total_bytes() as usize];
                    let _ = decoder.read_image(&mut buf);

                    let _ = fancy_regex::Regex::new(str);
                }
            }
            _ => unreachable!(),
        }
    }
}

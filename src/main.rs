mod cli;

use cli::{CliArgs, Quality};
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;
use webp::Encoder as WebpEncoder;

fn main() {
    let cli = match CliArgs::new() {
        Some(c) => c,
        None => return,
    };

    for (ipath, opath) in cli.files.iter() {
        to_webp(&ipath, &opath, cli.quality);
    }
}

fn read_image(ipath: &PathBuf) -> Option<DynamicImage> {
    match ImageReader::open(ipath) {
        Ok(read_img) => match read_img.decode() {
            Ok(decoded) => Some(decoded),
            Err(_e) => {
                eprintln!(
                    "towebp: Failed to decode image '{}'",
                    ipath.to_string_lossy()
                );
                None
            }
        },
        Err(_e) => {
            eprintln!("towebp: Failed to read image '{}'", ipath.to_string_lossy());
            None
        }
    }
}

fn to_webp(ipath: &PathBuf, opath: &PathBuf, quality: Quality) {
    if let Some(img) = read_image(ipath) {
        let encoder = match WebpEncoder::from_image(&img) {
            Ok(enc) => enc,
            Err(err) => {
                eprintln!(
                    "towebp: Could not create encoder for image '{}'. Got error {}",
                    ipath.to_string_lossy(),
                    err
                );
                return;
            }
        };

        let webp = match quality {
            Quality::Lossy { quality } => encoder.encode(quality),
            Quality::Lossless => encoder.encode_lossless(),
        };

        if let Err(_) = fs::write(opath, webp.iter()) {
            eprintln!(
                "towebp: Failed to write out webp for '{}'",
                ipath.to_string_lossy()
            );
        }
    }
}

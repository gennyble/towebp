mod cli;

use cli::CliArgs;
use image::io::Reader as ImageReader;
use webp::Encoder as WebpEncoder;
use std::fs;
use std::path::PathBuf;

fn main() {
	let cli = match CliArgs::new() {
		Some(c) => c,
		None => return
	};

    for (ipath, opath) in cli.files.iter() {
		to_webp(&ipath, &opath, cli.quality);
	}
}

fn to_webp(ipath: &PathBuf, opath: &PathBuf, quality: f32) {
    let img = match ImageReader::open(ipath) {
		Ok(read_img) => match read_img.decode() {
			Ok(decoded) => decoded,
			Err(_e) => {
				eprintln!("towebp: Failed to decode image '{}'", ipath.to_string_lossy());
				return;
			}
		},
		Err(_e) => {
			eprintln!("towebp: Failed to read image '{}'", ipath.to_string_lossy());
			return;
		}
	};

	let webp = WebpEncoder::from_image(&img).encode(quality);
	if let Err(_) = fs::write(opath, webp.iter()) {
		eprintln!("towebp: Failed to write out webp for '{}'", ipath.to_string_lossy());
	}
}
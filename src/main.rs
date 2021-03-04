mod cli;

use cli::CliArgs;
use image::io::Reader as ImageReader;
use webp::Encoder as WebpEncoder;
use std::fs;

fn main() {
	let cli = match CliArgs::new() {
		Some(c) => c,
		None => return
	};

    let img = ImageReader::open(cli.ipath)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

	let webp = WebpEncoder::from_image(&img).encode(cli.quality);
	fs::write(cli.opath, webp.iter()).expect("Failed to write webp to file");
}

use image::io::Reader as ImageReader;
use webp::Encoder as WebpEncoder;
use std::fs;

fn main() {
    let img = ImageReader::open("image.png")
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

	let webp = WebpEncoder::from_image(&img).encode(80.0);
	fs::write("image.webp", webp.iter()).expect("Failed to write webp to file");
}

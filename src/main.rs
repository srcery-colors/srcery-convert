extern crate image;
// use image::{Pixel, Rgba};

mod palette;

fn main() {

    let palette = match palette::read_palette() {
        Ok(palette) => palette,
        Err(e) => panic!("Failed to read palette file\n {}", e)
    };

    println!("{:?}", palette.green.rgb);

    // convert();
}

fn convert() {
    let mut img = image::open("example.png")
        .expect("Failed to read input image")
        .to_rgb8();
}

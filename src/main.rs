extern crate image;
use std::collections::HashMap;
use image::Rgb;

mod palette;
use palette::Color;

fn main() {
    let palette = match palette::read_palette() {
        Ok(palette) => palette,
        Err(e) => panic!("Failed to read palette file\n {}", e),
    };

    // println!("{:?}", palette.get("black").unwrap());
    convert(&palette);
}

fn convert(palette: &HashMap<String, Color>) {
    let mut img = image::open("example.png")
        .expect("Failed to read input image")
        .to_rgb8();

    for (x, y, px) in img.enumerate_pixels() {
        let d = &palette.get("green").unwrap().distance(&px);
        println!("{}", d);
    }
}

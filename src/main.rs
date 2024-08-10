extern crate image;
use image::{Rgb, RgbImage};
use std::collections::HashMap;
mod palette;
use palette::Color;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    input: String,

    #[arg(required = true)]
    output: String
}

fn main() {
    let palette = match palette::read_palette() {
        Ok(palette) => palette,
        Err(e) => panic!("Failed to read palette file\n {}", e),
    };
    let args = Args::parse();
    convert(&palette, &args.input, &args.output);
}

fn convert(palette: &HashMap<String, Color>, input_file: &str, output_file: &str) {
    let img = image::open(input_file)
        .expect("Failed to read input image")
        .to_rgb8();
    let mut buffer = RgbImage::new(img.width(), img.height());

    for (x, y, px) in img.enumerate_pixels() {
        let mut nearest = palette.get("black").unwrap();
        let mut distance = nearest.distance(px);
        for (_, color) in palette.iter() {
            if color.distance(px) < distance {
                nearest = color;
                distance = nearest.distance(px);
            }
        }
        let new = Rgb(nearest.rgb);
        buffer.put_pixel(x, y, new);
    }

    buffer
        .save(output_file)
        .expect("Failed to store output image");
}

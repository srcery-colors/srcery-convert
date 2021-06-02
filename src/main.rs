extern crate image;
use image::{Rgb, RgbImage};
use std::collections::HashMap;
mod palette;
use palette::Color;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let palette = match palette::read_palette() {
        Ok(palette) => palette,
        Err(e) => panic!("Failed to read palette file\n {}", e),
    };

    let matches = App::new("srcery-convert")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("OUTPUT")
                .about("Sets the output file")
                .required(true)
                .index(2),
        )
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    convert(&palette, input_file, output_file);
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

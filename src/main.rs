extern crate image;
use std::collections::HashMap;
use image::Rgb;

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
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::new("OUTPUT")
            .about("Sets the output file")
            .required(true)
            .index(2))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    convert(&palette, input_file, output_file);
}

fn convert(palette: &HashMap<String, Color>, input_file: &str, output_file: &str) {
    let mut img = image::open(input_file)
        .expect("Failed to read input image")
        .to_rgb8();

    for (x, y, px) in img.clone().enumerate_pixels() {
        let mut nearest = palette.get("black").unwrap();
        for (_, color) in palette.iter() {
            if color.distance(px) < nearest.distance(px) {
                nearest = color;
            }
        }
        let replacement = Rgb(nearest.rgb);
        img.put_pixel(x, y, replacement);
    }

    img.save(output_file).expect("Failed to store output image");
}

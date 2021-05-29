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
    convert(&palette);
}

fn convert(palette: &HashMap<String, Color>) {
    let mut img = image::open("example.png")
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
        // println!("{:?}", nearest.rgb);
    }

    img.save("output.png").expect("Failed to store output image");
}

extern crate image;
use image::Rgb;

mod palette;
use palette::Palette;

fn main() {
    let palette = match palette::read_palette() {
        Ok(palette) => palette,
        Err(e) => panic!("Failed to read palette file\n {}", e),
    };


    convert(&palette);
}

fn distance(rgb1: &Vec<u8>, rgb2: &Rgb<u8>) -> f32 {
    let (r1, g1, b1) = (rgb1[0] as f32, rgb1[1] as f32, rgb1[2] as f32);
    let (r2, g2, b2) = (rgb2[0] as f32, rgb2[1] as f32, rgb2[2] as f32);
    (((r2 - r1) * 0.30).powi(2) + ((g2 - g1) * 0.59).powi(2) + ((b2 - b1) * 0.11).powi(2)).sqrt()
}

fn convert(palette: &Palette) {
    let mut img = image::open("example.png")
        .expect("Failed to read input image")
        .to_rgb8();

    // let new = &img
    //     .enumerate_pixels()
    //     .map(|(x, y, px)|)

    for (x, y, px) in img.enumerate_pixels() {
        let d = distance(&palette.green.rgb, &px);
        println!("{}", d);
    }
}

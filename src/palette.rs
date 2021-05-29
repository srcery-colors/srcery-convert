use std::io::Error;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use std::collections::HashMap;
use image::Rgb;

#[derive(Debug, Deserialize)]
pub struct Color {
    pub hex: String,
    pub rgb: [u8; 3],
    pub hsl: Vec<u32>
}

impl Color {
    pub fn distance(&self, other: &Rgb<u8>) -> f32 {
        let (r1, g1, b1) = (self.rgb[0] as f32, self.rgb[1] as f32, self.rgb[2] as f32);
        let (r2, g2, b2) = (other[0] as f32, other[1] as f32, other[2] as f32);
        (((r2 - r1) * 0.30).powi(2) + ((g2 - g1) * 0.59).powi(2) + ((b2 - b1) * 0.11).powi(2)).sqrt()
    }
}

pub fn read_palette() -> Result<HashMap<String, Color>, Error> {
    let mut file = File::open("palette.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let palette: HashMap<String, Color> = toml::from_str(&buffer)?;
    Ok(palette)
}


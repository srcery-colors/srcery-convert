use std::io::Error;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Palette {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub orange: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
    pub bright_orange: Color,
    pub teal: Color,
    pub hard_black: Color,
    pub xgray1: Color,
    pub xgray2: Color,
    pub xgray3: Color,
    pub xgray4: Color,
    pub xgray5: Color,
    pub xgray6: Color
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub hex: String,
    pub rgb: Vec<u32>,
    pub hsl: Vec<u32>
}

pub fn read_palette() -> Result<Palette, Error> {
    let mut file = File::open("palette.toml")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let palette: Palette = toml::from_str(&buffer)?;
    Ok(palette)
}


use std::io::{self, Read};

#[derive(Debug)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8
}

impl Color {

    pub fn new() -> Color {
        Color { red: 0, green: 0, blue: 0 }
    }

    pub fn parse(data: u16) -> Result<Color, io::Error> {
        let mut color = Self::new();

        let blue  = ((data & (0x1f <<  0)) >>  0) as u8;
        let green = ((data & (0x1f <<  5)) >>  5) as u8;
        let red   = ((data & (0x1f << 10)) >> 10) as u8;

        color.blue  = (blue  << 3) | (blue  >> 5);
        color.green = (green << 3) | (green >> 5);
        color.red   = (red   << 3) | (red   >> 5);

        Ok(color)
    }

    pub fn as_rgb(&self) -> Vec<u8> {
        vec!(self.red, self.green, self.blue)
    }
}

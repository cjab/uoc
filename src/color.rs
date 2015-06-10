pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {

    fn new() -> Color {
        Color { red: 0, green: 0, blue: 0 }
    }

    fn parse(buf: &[u8]) -> Result<Color, i::Error> {
        let mut cursor = io::Cursor::new(buf);
        let color_data = try!(cursor.read_u16::<LittleEndian>());
        let mut color = Self::new();
        color.blue  = color_data & (0x1f <<  0);
        color.green = color_data & (0x1f <<  5);
        color.red   = color_data & (0x1f << 10);
        Ok(color)
    }
}


pub struct Texture {
    pub width:  i32,
    pub height: i32,
    pub colors: Vec<Color>
}


impl Texture {
    pub fn parse(width: i32, height: i32, buf: &[u8]) -> Result<Texture, io::Error> {
        let colors_buf = buf.chunks(size_of::<Color>());
        Texture {
            width:  width,
            height: height,
            colors: colors_buf.map(Color::parse).collect()
        }
    }
}

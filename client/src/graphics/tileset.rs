use image::{ImageBuffer, Rgb};
use std::path::Path;

pub struct TileSet {
    pub img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    glyph_size: (u16, u16),
    glyph_padding: (u16, u16),
}

impl TileSet {
    pub fn new<P: AsRef<Path>>(path: P, glyph_size: (u16, u16), glyph_padding: (u16, u16)) -> Self {
        let img = image::open(path).unwrap().to_rgb();
        TileSet {
            img,
            glyph_padding,
            glyph_size,
        }
    }

    pub fn ratio(&self) -> f64 {
        let x = self.glyph_size.0 as f64;
        let y = self.glyph_size.1 as f64;
        x / y
    }

    pub fn get_glyph(&self, id: u32) -> [[f32; 2]; 4] {
        let a_w = self.img.dimensions().0 / (self.glyph_size.0 + self.glyph_padding.0) as u32;
        let a_h = self.img.dimensions().1 / (self.glyph_size.1 + self.glyph_padding.1) as u32;
        let size_x = self.glyph_size.0 as f32 / self.img.dimensions().0 as f32;
        let size_y = self.glyph_size.1 as f32 / self.img.dimensions().1 as f32;
        let idx = id % a_w;
        let idy = (-(id as i64 / a_h as i64) + a_h as i64) as u32 - 1;
        let start_x = idx as f32 / a_w as f32;
        let start_y = idy as f32 / a_h as f32;
        let end_x = start_x + size_x;
        let end_y = start_y + size_y;
        [
            [start_x, start_y],
            [start_x, end_y],
            [end_x, end_y],
            [end_x, start_y],
        ]
    }
}

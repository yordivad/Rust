extern crate png;

use crate::ui::canvas::{Canvas, Color, Drawing};
use std::fs::File;

pub struct Image {
    name: &'static str,
}

impl Image {
    pub fn new(name: &'static str) -> Self {
        Image { name }
    }
}

impl Drawing for Image {
    fn draw(&self, canvas: &mut Canvas) -> () {
        let decoder = png::Decoder::new(File::open(self.name).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buffer = vec![0; info.buffer_size()];
        reader.next_frame(&mut buffer).unwrap();

        let img: Vec<Color> = buffer
            .chunks(3)
            .map(|p| Color::from(p[0], p[1], p[2]))
            .collect();

        for x in 0..info.width {
            for y in 0..info.height {
                let c = img[(x + (y * info.width)) as usize];
                canvas.paint(x as i32, y as i32 * -1, 0, c)
            }
        }
    }
}

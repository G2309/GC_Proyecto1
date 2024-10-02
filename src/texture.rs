use image::{ImageReader, DynamicImage, GenericImageView};
use crate::color::Color;

pub struct Texture {
    image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        Texture { image: img, width, height }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> Color {
        if x >= self.width || y >= self.height {
            return Color::new(255, 0, 0);
        }

        let pixel = self.image.get_pixel(x, y);
        Color::new(pixel[0], pixel[1], pixel[2])
    }
}


use std::{io::Cursor, ops::Deref};

use image::{io::Reader as ImageReader, RgbaImage};

pub struct Image {
    image: RgbaImage,
}

impl Image {
    pub fn from_raw(width: u32, height: u32, buf: Vec<u8>) -> Self {
        let image = RgbaImage::from_raw(width, height, buf).unwrap();

        Self { image }
    }

    pub fn from_size(width: u32, height: u32) -> Self {
        let image = RgbaImage::new(width, height);

        Self { image }
    }

    pub fn from_image(image: &[u8]) -> anyhow::Result<Self> {
        let image = ImageReader::new(Cursor::new(image)).with_guessed_format()?.decode()?;

        Ok(Self { image: image.into_rgba8() })
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn bytes_per_pixel(&self) -> u32 {
        4
    }

    pub fn buffer(&self) -> &[u8] {
        self.image.as_raw()
    }
}

pub struct Canvas {
    canvas: Image,
}

impl Canvas {
    pub fn from_raw(width: u32, height: u32, buf: Vec<u8>) -> Self {
        Self {
            canvas: Image::from_raw(width, height, buf),
        }
    }

    pub fn from_size(width: u32, height: u32) -> Self {
        Self {
            canvas: Image::from_size(width, height),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(&mut self, dx: u32, dy: u32, w: u32, h: u32, src: &Image, sx: u32, sy: u32) {
        for j in dy..(dy + h) {
            for i in dx..(dx + w) {
                if i >= self.canvas.width() || j >= self.canvas.height() {
                    continue; // TODO remove this
                }
                self.canvas.image.put_pixel(i, j, *src.image.get_pixel(i - dx + sx, j - dy + sy));
            }
        }
    }
}

impl Deref for Canvas {
    type Target = Image;

    fn deref(&self) -> &Self::Target {
        &self.canvas
    }
}

use core::{mem::size_of, ops::Deref};
use std::ops::DerefMut;

use bytemuck::{cast_slice, pod_collect_to_vec, Pod};
use image::io::Reader as ImageReader;
use num_traits::{Num, Zero};

#[derive(Clone, Copy)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait Image {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn bytes_per_pixel(&self) -> u32;
    fn get_pixel(&self, x: u32, y: u32) -> Color;
    fn raw(&self) -> &[u8];
    fn colors(&self) -> Vec<Color>;
    fn into_image_buffer(self: Box<Self>) -> Option<Box<dyn ImageBuffer>> {
        None
    }
}

pub trait ImageBuffer: Image {
    fn put_pixels(&mut self, x: u32, y: u32, width: u32, colors: &[Color]);
}

pub trait PixelType {
    type DataType: Copy + Pod + Num;
    fn from_color(color: Color) -> Self::DataType;
    fn to_color(raw: Self::DataType) -> Color;
}

pub struct Rgb565Pixel {}

impl PixelType for Rgb565Pixel {
    type DataType = u16;

    fn from_color(color: Color) -> Self::DataType {
        let r = (color.r as u16) >> 3;
        let g = (color.g as u16) >> 2;
        let b = (color.b as u16) >> 3;

        (r << 11) | (g << 5) | b
    }

    fn to_color(raw: Self::DataType) -> Color {
        let r = ((raw >> 11) & 0x1f) as u8;
        let g = ((raw >> 5) & 0x3f) as u8;
        let b = (raw & 0x1f) as u8;

        let r = ((r as u32 * 255 + 15) / 31) as u8;
        let g = ((g as u32 * 255 + 31) / 63) as u8;
        let b = ((b as u32 * 255 + 15) / 31) as u8;

        Color { a: 0xff, r, g, b }
    }
}

pub struct Rgb8Pixel {}

impl PixelType for Rgb8Pixel {
    type DataType = u32;

    fn from_color(color: Color) -> Self::DataType {
        (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32
    }

    fn to_color(raw: Self::DataType) -> Color {
        let r = ((raw >> 16) & 0xff) as u8;
        let g = ((raw >> 8) & 0xff) as u8;
        let b = (raw & 0xff) as u8;

        Color { a: 0xff, r, g, b }
    }
}

pub struct ArgbPixel {}

impl PixelType for ArgbPixel {
    type DataType = u32;

    fn from_color(color: Color) -> Self::DataType {
        (color.a as u32) << 24 | (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32
    }

    fn to_color(raw: Self::DataType) -> Color {
        let a = ((raw >> 24) & 0xff) as u8;
        let r = ((raw >> 16) & 0xff) as u8;
        let g = ((raw >> 8) & 0xff) as u8;
        let b = (raw & 0xff) as u8;

        Color { a, r, g, b }
    }
}

pub struct AbgrPixel {}

impl PixelType for AbgrPixel {
    type DataType = u32;

    fn from_color(color: Color) -> Self::DataType {
        (color.a as u32) << 24 | (color.b as u32) << 16 | (color.g as u32) << 8 | color.r as u32
    }

    fn to_color(raw: Self::DataType) -> Color {
        let a = ((raw >> 24) & 0xff) as u8;
        let b = ((raw >> 16) & 0xff) as u8;
        let g = ((raw >> 8) & 0xff) as u8;
        let r = (raw & 0xff) as u8;

        Color { a, r, g, b }
    }
}

pub struct VecImageBuffer<T>
where
    T: PixelType,
{
    width: u32,
    height: u32,
    data: Vec<T::DataType>,
}

impl<T> VecImageBuffer<T>
where
    T: PixelType,
{
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![T::DataType::zero(); (width * height) as usize],
        }
    }

    pub fn from_raw(width: u32, height: u32, raw: Vec<T::DataType>) -> Self {
        Self { width, height, data: raw }
    }
}

impl<T> Image for VecImageBuffer<T>
where
    T: PixelType + 'static,
{
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn bytes_per_pixel(&self) -> u32 {
        size_of::<T::DataType>() as u32
    }

    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let raw = self.data[(y * self.width + x) as usize];

        T::to_color(raw)
    }

    fn raw(&self) -> &[u8] {
        cast_slice(&self.data)
    }

    fn colors(&self) -> Vec<Color> {
        self.data.iter().map(|&x| T::to_color(x)).collect()
    }

    fn into_image_buffer(self: Box<Self>) -> Option<Box<dyn ImageBuffer>> {
        Some(self)
    }
}

impl<T> ImageBuffer for VecImageBuffer<T>
where
    T: PixelType + 'static,
{
    fn put_pixels(&mut self, x: u32, y: u32, width: u32, colors: &[Color]) {
        for (i, color) in colors.iter().enumerate() {
            let x = x + (i as u32 % width);
            let y = y + (i as u32 / width);

            if x >= self.width || y >= self.height {
                continue;
            }

            let raw = T::from_color(*color);

            self.data[(y * self.width + x) as usize] = raw;
        }
    }
}

pub struct Canvas<T>
where
    T: Deref<Target = dyn ImageBuffer> + DerefMut,
{
    pub image_buffer: T, // TODO remove pub
}

impl<T> Canvas<T>
where
    T: Deref<Target = dyn ImageBuffer> + DerefMut,
{
    pub fn new(image_buffer: T) -> Self {
        Self { image_buffer }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(&mut self, dx: u32, dy: u32, w: u32, h: u32, src: &dyn Image, sx: u32, sy: u32) {
        for y in 0..h {
            for x in 0..w {
                if sx + x >= src.width() || sy + y >= src.height() {
                    continue;
                }
                if dx + x >= self.image_buffer.width() || dy + y >= self.image_buffer.height() {
                    continue;
                }

                let color = src.get_pixel(sx + x, sy + y);

                if color.a == 0 {
                    continue; // TODO alpha blending
                }

                self.put_pixel(dx + x, dy + y, color);
            }
        }
    }

    // TODO change it to bresenham's or something..
    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: Color) {
        let dx = ((x2 + 1) as f32) - (x1 as f32);
        let dy = ((y2 + 1) as f32) - (y1 as f32);

        let mut x = x1 as f32;
        let mut y = y1 as f32;

        let step = dx.abs().max(dy.abs());

        let dx = dx / step;
        let dy = dy / step;

        for _ in 0..step as u32 {
            if x >= self.image_buffer.width() as f32 || y >= self.image_buffer.height() as f32 {
                continue;
            }
            self.put_pixel(x as u32, y as u32, color);

            x += dx;
            y += dy;
        }
    }

    pub fn draw_text(&mut self, string: &str, x: u32, y: u32) {
        // TODO can we draw directly on canvas? without it AA blending looks horrible..
        use piet::{ImageFormat, RenderContext, Text, TextLayout, TextLayoutBuilder};
        use piet_common::Device;

        let mut device = Device::new().unwrap();
        let mut bitmap_target = device
            .bitmap_target(self.image_buffer.width() as _, self.image_buffer.height() as _, 1.0)
            .unwrap();
        let mut context = bitmap_target.render_context();

        let text_layout = context.text().new_text_layout(string.to_owned()).build().unwrap();
        let bound = text_layout.image_bounds();

        context.draw_text(&text_layout, (0.0, 0.0));

        context.finish().unwrap();
        drop(context);

        let image_buf = bitmap_target.to_image_buf(ImageFormat::RgbaPremul).unwrap();

        let image = VecImageBuffer::<ArgbPixel>::from_raw(
            image_buf.width() as _,
            image_buf.height() as _,
            cast_slice(image_buf.raw_pixels()).to_vec(),
        );

        self.draw(x, y, bound.width() as _, bound.height() as _, &image, 0, 0);
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        for x in x..x + w {
            if x >= self.image_buffer.width() {
                continue;
            }
            self.put_pixel(x, y, color);
            self.put_pixel(x, y + h - 1, color);
        }
        for y in y..y + h {
            if y >= self.image_buffer.height() {
                continue;
            }
            self.put_pixel(x, y, color);
            self.put_pixel(x + w - 1, y, color);
        }
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        for y in y..y + h {
            for x in x..x + w {
                if x >= self.image_buffer.width() || y >= self.image_buffer.height() {
                    continue;
                }
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.image_buffer.put_pixels(x, y, 1, &[color]);
    }
}

pub fn decode_image(data: &[u8]) -> anyhow::Result<Box<dyn Image>> {
    use std::io::Cursor;

    let image = ImageReader::new(Cursor::new(&data)).with_guessed_format()?.decode()?;
    let rgba = image.into_rgba8();

    let data = rgba.pixels().flat_map(|x| [x.0[2], x.0[1], x.0[0], x.0[3]]).collect::<Vec<_>>();

    Ok(Box::new(VecImageBuffer::<ArgbPixel>::from_raw(
        rgba.width(),
        rgba.height(),
        pod_collect_to_vec(&data),
    )) as Box<_>)
}

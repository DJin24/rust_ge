use sdl2::rect::Rect;
use sdl2::render;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

pub struct Sprite<'a> {
    shape: Rect, // has position and size of sprite
    surface: Surface<'a>, // probably a better way to store a bitmap -- maybe we just store the whole sdl2_surface?
    pixels: Box<[u8; 1024]>
}

impl<'a> Sprite<'a> {
    pub fn rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        let mut data = Box::new([8u8; 4 * 256]);
        let surf = sdl2::surface::Surface::from_data(data.as_mut(), 16, 16, 16*4, sdl2::pixels::PixelFormatEnum::RGBA8888).unwrap();
        Self {
            shape: sdl2::rect::Rect::new(0, 0, 16, 16),
            surface: surf,
            pixels: data
        }
    }
    pub fn circle(x: i32, y: i32, radius: u32, color: Color) -> Self {
        todo!();
    }
    pub fn as_sdl_surface(&'a self) -> &Surface<'a> {
        &self.surface
    }
}
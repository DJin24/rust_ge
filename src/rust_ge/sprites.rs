use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::surface::Surface;

pub struct Sprite<'a> {
    shape: Rect,          // has position and size of sprite
    surface: Surface<'a>, // probably a better way to store a bitmap -- maybe we just store the whole sdl2_surface?
}

impl<'a> Sprite<'a> {
    pub fn rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        todo!();
    }
    pub fn circle(x: i32, y: i32, radius: u32, color: Color) -> Self {
        todo!();
    }
    pub fn as_sdl_surface(&'a self) -> &Surface<'a> {
        &self.surface
    }
}

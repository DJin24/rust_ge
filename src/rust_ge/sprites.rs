use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::surface::Surface;

pub enum ShapeTypes {
    Rect,
    FilledRect,
    Point,
}

pub struct Sprite<'a> {
    shape_type: ShapeTypes,
    shape: Rect,
    // has position and size of sprite
    color: Color,
    surface: Surface<'a>, // probably a better way to store a bitmap -- maybe we just store the whole sdl2_surface?
}

impl<'a> Sprite<'a> {
    pub fn rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::Rect,
            shape: Rect::new(x, y, width, height),
            color: color,
            surface: Surface::new(width, height, PixelFormatEnum::RGBA8888).unwrap(), // TODO this should probably be handled
        }
    }
    pub fn filled_rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::FilledRect,
            shape: Rect::new(x, y, width, height),
            color: color,
            surface: Surface::new(width, height, PixelFormatEnum::RGBA8888).unwrap(), // TODO this should probably be handled
        }
    }

    pub fn circle(x: i32, y: i32, radius: u32, color: Color) -> Self {
        todo!();
    }
    pub fn as_sdl_surface(&'a self) -> &Surface<'a> {
        &self.surface
    }

    pub fn shape_type(&self) -> &ShapeTypes {
        &self.shape_type
    }

    pub fn shape(&self) -> Rect {
        self.shape
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

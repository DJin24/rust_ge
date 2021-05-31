use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::surface::Surface;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

pub enum ShapeTypes {
    Rect,
    FilledRect,
    Point,
}

pub struct Sprite {
    shape_type: ShapeTypes,
    shape: Rect,
    // has position and size of sprite
    color: Color,
    pixels: Box<[u8; 1024]>,
}

impl Sprite {
    pub fn rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::Rect,
            shape: Rect::new(x, y, width, height),
            color: color,
            pixels: Box::new([8u8; 4 * 256]),
        }
    }
    pub fn filled_rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::FilledRect,
            shape: Rect::new(x, y, width, height),
            color: color,
            pixels: Box::new([8u8; 4 * 256]),
        }
    }

    pub fn circle(x: i32, y: i32, radius: u32, color: Color) -> Self {
        todo!();
    }
    pub fn as_sdl_surface(&mut self) -> Surface {
        sdl2::surface::Surface::from_data(
            self.pixels.as_mut(),
            16,
            16,
            16 * 4,
            sdl2::pixels::PixelFormatEnum::RGBA8888,
        )
        .unwrap()
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

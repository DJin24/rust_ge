use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect, Point};
use sdl2::render;
use sdl2::surface::Surface;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShapeTypes {
    Rect,
    FilledRect,
    Line,
    Point,
}

pub struct Sprite {
    shape_type: ShapeTypes,
    shape: Rect,
    // has position and size of sprite
    color: Color,
    // There must be some better way of doing this within the Rect struct,
    // or maybe eliminate the shape field
    start: Point,
    end: Option<Point>
}

impl Sprite {
    pub fn rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::Rect,
            shape: Rect::new(x, y, width, height),
            color: color,
            start: Point::new(x, y),
            end: Some(Point::new(x + width as i32, y + height as i32)),
        }
    }
    pub fn filled_rectangle(x: i32, y: i32, height: u32, width: u32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::FilledRect,
            shape: Rect::new(x, y, width, height),
            color: color,
            start: Point::new(x, y),
            end: Some(Point::new(x + width as i32, y + height as i32)),
        }
    }

    // also have a line from points? Would require a public point type
    pub fn line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) -> Self {
        Sprite {
            shape_type: ShapeTypes::Line,
            shape: Rect::new(x1, y1, (x1 - x2).abs() as u32, (y1 - y2).abs() as u32),
            color: color,
            start: Point::new(x1, y1),
            end: Some(Point::new(x2, y2))
        }
    }

    // This basically needs to be drawn manually
    pub fn circle(x: i32, y: i32, radius: u32, color: Color) -> Self {
        todo!();
    }

    // Deprecated
    // pub fn as_sdl_surface(&mut self) -> Surface {
    //     sdl2::surface::Surface::from_data(
    //         self.pixels.as_mut(),
    //         16,
    //         16,
    //         16 * 4,
    //         sdl2::pixels::PixelFormatEnum::RGBA8888,
    //     )
    //         .unwrap()
    // }

    pub fn shape_type(&self) -> &ShapeTypes {
        &self.shape_type
    }

    pub fn shape(&self) -> Rect {
        self.shape
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Option<Point> {
        self.end
    }
}

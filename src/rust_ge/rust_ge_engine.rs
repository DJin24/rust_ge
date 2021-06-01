extern crate sdl2;

use crate::rust_ge::rust_ge_base::AbstractGame;
use crate::rust_ge::sprites::{Sprite, ShapeTypes};
use crate::rust_ge::frame_rate::FrameRate;
use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use ::sdl2::pixels::Color;
use ::sdl2::render::Canvas;
use ::sdl2::video::Window;
use ::sdl2::EventPump;
use ::std::time::Duration;
use ::std::collections::HashSet;
use ::std::rc::Rc;
use ::std::cell::RefCell;
use crate::rust_ge::rust_ge_event::{Key, Mouse_button, Posn};


struct EngineData {
    canvas: Canvas<Window>,
    frame_rate: FrameRate,
    event_pump: EventPump,
}

pub struct Engine {
    data: RefCell<EngineData>,
}

// Instead of an engine owning a game, it just is created and then accepts a game as a parameter to run
impl Engine {
    pub fn new(fr: usize) -> Self {
        let mut frame_rate = FrameRate::new(fr);

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let data = RefCell::new(EngineData {
            canvas,
            frame_rate,
            event_pump,
        });
        Self {
            data
        }
    }

    pub fn run<Game: AbstractGame + Sized>(&self, game: &mut Game) {
        game.on_start();
        let mut data = self.data.borrow_mut();
        data.canvas.set_draw_color(Color::RGB(0, 0, 0));
        data.canvas.clear();
        data.canvas.present();
        let mut dt = Duration::from_secs(0);
        'running: loop {
            data.canvas.set_draw_color(Color::BLACK);
            data.canvas.clear();
            
            game.on_frame(dt);
            // maybe &Sprite, though might be confusing with lifetimes
            // Rc's would solve the dropping issue, but could lead to a memory leak
            let mut sprites = Vec::<Sprite>::new();
            game.draw(dt, &mut sprites);

            // Doesn't like this, it looks like the sprites are dropped when the canvas depends on them to stay there
            // let surfaces = sprites.iter_mut().map(|sprite| sprite.as_sdl_surface());
            let texture_creator = data.canvas.texture_creator();
            for mut sprite in sprites {
                match sprite.shape_type() {
                    ShapeTypes::Rect => {
                        data.canvas.set_draw_color(sprite.color());
                        data.canvas.draw_rect(sprite.shape());
                    },
                    ShapeTypes::FilledRect => {
                        data.canvas.set_draw_color(sprite.color());
                        data.canvas.fill_rect(Some(sprite.shape()));
                    },
                    ShapeTypes::Line => {
                        data.canvas.set_draw_color(sprite.color());
                        data.canvas.draw_line(sprite.start(), sprite.end().unwrap());
                    },
                    ShapeTypes::Point => {
                        data.canvas.set_draw_color(sprite.color());
                        data.canvas.draw_point(sprite.start());
                    }
                    //_ => (),
                };

            }
            for event in data.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => Engine::handle_events(game, event),
                }
            }
            data.canvas.present();

            dt = data.frame_rate.wait_for_next_frame();
        }
        game.on_quit()
    }

    fn handle_events<Game: AbstractGame + Sized>(game: &mut Game, event: Event) {
        match event {
            Event::TextInput { text, .. } => {
                for c in text.chars() {
                    if let Some(key) = Key::map_char(c) {
                        game.on_key(key);
                    }
                }
            }
            Event::KeyDown {
                keycode: Some(key_code),
                repeat: repeat,
                ..
            } => {
                if let Some(key) = Key::map_key(key_code) {
                    println!("{:?}", key);
                    if !repeat {
                        println!("Pressed -> {:?}", key);
                        game.on_key_down(key);
                    }
                    if !key.is_textual() {
                        game.on_key(key);
                    }

                    println!("Held -> {:?}", key);
                };
            }
            Event::KeyUp {
                keycode: Some(key_code),
                ..
            } => {
                if let Some(key) = Key::map_key(key_code) {
                    println!("Released -> {:?}", key);
                    game.on_key_up(key);
                };
            }
            Event::MouseButtonDown {
                mouse_btn, x, y, ..
            } => {
                if let Some(mouse_button) = Mouse_button::map_button(mouse_btn) {
                    println!("Mouse Down -> {:?}", mouse_button);
                    game.on_mouse_down(mouse_button, Posn { x, y });
                }
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                if let Some(mouse_button) = Mouse_button::map_button(mouse_btn) {
                    println!("Mouse Up -> {:?}", mouse_button);
                    game.on_mouse_up(mouse_button, Posn { x, y });
                }
            }
            Event::MouseMotion { x, y, .. } => game.on_mouse_move(Posn { x, y }),
            _ => {}
        }
    }
}

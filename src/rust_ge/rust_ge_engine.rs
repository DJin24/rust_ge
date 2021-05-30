extern crate sdl2;

use ::sdl2::event::Event;
use crate::rust_ge::rust_ge_base::AbstractGame;
use ::sdl2::keyboard::Keycode;
use ::sdl2::pixels::Color;
use ::std::time::Duration;
use ::std::collections::HashSet;
use crate::rust_ge::sprites::Sprite;
use crate::rust_ge::frame_rate::FrameRate;
use ::std::rc::Rc;
use ::sdl2::render::Canvas;
use ::sdl2::video::Window;
use ::std::cell::RefCell;
use ::sdl2::EventPump;

struct EngineData {
    canvas: Canvas<Window>,
    frame_rate: FrameRate,
    event_pump: EventPump
}

pub struct Engine {
    data: RefCell<EngineData>
}

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
        let data = RefCell::new(EngineData{
            canvas, frame_rate, event_pump
        });
        Self {
            data
        }
    }

    pub fn run<Game: AbstractGame + Sized>(&self, game: &mut Game) {
        game.on_start();
        let mut data = self.data.borrow_mut();
        data.canvas.set_draw_color(Color::RGB(0, 255, 255));
        data.canvas.clear();
        data.canvas.present();
        let mut i = 0;
        let mut dt = Duration::from_secs(0);
        'running: loop {
            data.canvas.clear();
            let mut sprites = Vec::<Sprite>::new(); // maybe &Sprite, though might be confusing with lifetimes
            game.draw(dt, &mut sprites);
            let surfaces = sprites.iter_mut().map(|sprite| sprite.as_sdl_surface());
            let texture_creator = data.canvas.texture_creator();
            for surface in surfaces {
                match texture_creator.create_texture_from_surface(surface){
                    Err(_) => panic!("failed to create texture on window"),
                    Ok(_texture) => ()
                };
            }
            for event in data.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => game.handle_events(event),
                }
            }
            data.canvas.present();
            
            dt = data.frame_rate.wait_for_next_frame();
        }
        game.on_quit()
    }
    
    fn handle_events(e: Event) {}
}
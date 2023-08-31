use std::time::Duration;

use crate::world::World;

pub mod render;
pub mod sdl;
pub mod update;

pub use render::*;
pub use sdl::*;
pub use update::*;

use sdl2::{event::Event, keyboard::Keycode};

pub struct App {
    pub world: World,
    pub settings: Settings,
}

impl App {
    pub fn init() -> Self {
        log::info!(target: "app", "Create and init app.");
        let mut world = World::new();
        let settings = Settings::default();

        world.generate();

        log::info!(target: "app", "The creation and initialization of the application was successful.");
        Self { world, settings }
    }

    pub fn run(&mut self) {
        log::info!(target: "app_run", "Launching the application.");

        let stop_frame = Duration::new(0, 1_000_000_000u32 / crate::constants::FPS);
        let mut sdl = SDL::init(&self.settings);

        log::info!(target: "app_run", "Starting the main application cycle.");
        'running: loop {
            sdl.canvas.set_draw_color((0x15, 0x15, 0x17));
            sdl.canvas.clear();

            if event_handler(&sdl) {
                break 'running;
            }

            let world_read = self.world.clone();
            self.update(&world_read);
            App::render(&world_read, &mut sdl);

            sdl.canvas.present();

            if cfg!(test) {
                break 'running;
            }

            ::std::thread::sleep(stop_frame);
        }

        log::info!(target: "app_run", "Shutting down the application.");
    }
}

pub(self) fn event_handler(sdl: &SDL) -> bool {
    let mut event_pump = sdl.sdl_context.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        log::trace!(target: "app_event", "{:?}", event);

        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            _ => {}
        }
    }

    false
}

pub struct Settings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: String::from("Dryad"),
            size: (1200, 600),
        }
    }
}

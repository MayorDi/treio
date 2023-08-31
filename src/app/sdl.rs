use super::Settings;

pub struct SDL {
    pub sdl_context: sdl2::Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl SDL {
    pub fn init(settings: &Settings) -> Self {
        log::info!(target: "SDL", "Initialization of SDL.");

        let sdl_ctx = sdl2::init().expect("Error init SDL.");
        let video_subsystem = sdl_ctx.video().expect("Error init video_subsystem.");
        let window = video_subsystem
            .window(settings.title.as_str(), settings.size.0, settings.size.1)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("Error init window.");

        let canvas = window.into_canvas().build().expect("Error init canvas");

        log::info!(target: "SDL", "Initialization of SDL is complete.");
        Self {
            sdl_context: sdl_ctx.clone(),
            canvas,
        }
    }
}

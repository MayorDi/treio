#![cfg(target_os = "windows")]

use treio::app::App;

pub fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::builder().init();
    App::init().run();
}

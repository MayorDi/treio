//! All project traits are stored here.

use crate::{app::SDL, world::World};

/// Allows you to conduct a common render.
pub trait Render {
    fn render(&self, sdl: &mut SDL, idx: usize);
}

/// Sets behavioral features.
pub trait Behaviour {
    fn update(world_read: &World, world: &mut World, idx: usize);
}

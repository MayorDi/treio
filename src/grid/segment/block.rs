use crate::{
    traits::{Behaviour, Render},
    world::World,
};

/// `Block` - the essence, which is impassable.
#[derive(Debug, Clone, Default)]
pub struct Block {}

impl Render for Block {
    fn render(&self, sdl: &mut crate::app::SDL) {}
}

impl Behaviour for Block {
    fn update(world_read: &World, world: &mut World, idx: usize) {}
}

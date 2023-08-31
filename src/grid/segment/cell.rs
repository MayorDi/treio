use crate::{
    traits::{Behaviour, Render},
    world::World,
};

use super::Genome;

/// `Cell` is the main, executive essence of the simulation.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub energy: f32,
    pub genome: Genome,
}

impl Cell {
    /// Creates a new cell with a given amount of energy.
    pub fn new(energy: f32) -> Self {
        Self {
            energy,
            genome: Default::default(),
        }
    }
}

impl Render for Cell {
    fn render(&self, sdl: &mut crate::app::SDL) {}
}

impl Behaviour for Cell {
    fn update(world_read: &World, world: &mut World, idx: usize) {}
}

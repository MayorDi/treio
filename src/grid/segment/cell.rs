use sdl2::rect::Rect;

use crate::{
    traits::{Behaviour, Render},
    world::{World, get_pos}, constants::{SIZE_WORLD, SIZE_RECT_RENDER},
};

use super::Genome;

/// `Cell` is the main, executive essence of the simulation.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub energy: f32,
    pub is_seed: bool,
    pub genome: Genome,
}

impl Cell {
    /// Creates a new cell with a given amount of energy.
    pub fn new(energy: f32) -> Self {
        Self {
            energy,
            is_seed: false,
            genome: Default::default(),
        }
    }
}

impl Render for Cell {
    fn render(&self, sdl: &mut crate::app::SDL, idx: usize) {
        let pos = get_pos(idx, SIZE_WORLD[0]);
        let (x, y) = (pos.x as i32, pos.y as i32);
        let canvas = &mut sdl.canvas;
        let rect = Rect::new(
            x * SIZE_RECT_RENDER,
            SIZE_RECT_RENDER * (-y + 2*(SIZE_WORLD[1] as i32) - 1),
            SIZE_RECT_RENDER as u32,
            SIZE_RECT_RENDER as u32,
        );

        canvas.set_draw_color(if self.is_seed {
            (0xed, 0xf2, 0xa8)
        } else {
            (0x54, 0x92, 0x48)
        });
        canvas.fill_rect(rect).unwrap();
    }
}

impl Behaviour for Cell {
    fn update(world_read: &World, world: &mut World, idx: usize) {}
}

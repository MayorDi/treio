use sdl2::rect::Rect;

use crate::{
    constants::{SIZE_RECT_RENDER, SIZE_WORLD},
    traits::{Behaviour, Render},
    world::{get_neighbors_idxs, get_pos, World},
};

use super::{Genome, Segment};

const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;

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
            SIZE_RECT_RENDER * (-y + 2 * (SIZE_WORLD[1] as i32) - 1),
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
    fn update(world_read: &World, world: &mut World, idx: usize) {
        let neighbores = get_neighbors_idxs(idx);
        let cell = world.grid[idx].to_cell().unwrap();
        if cell.is_seed {
            if let Segment::Air = world_read.grid[neighbores[BOTTOM]] {
                world.grid[neighbores[BOTTOM]] = Segment::Cell(cell.clone());
                world.grid[idx] = Segment::Air;
            } else {
                world.grid[idx].to_cell().unwrap().is_seed = false;
            }
        }

        
    }
}

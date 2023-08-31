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
    pub lifetime: usize,
    pub next: i8,
    pub genome: Genome,
}

impl Cell {
    /// Creates a new cell with a given amount of energy.
    pub fn new(energy: f32) -> Self {
        Self {
            energy,
            is_seed: false,
            lifetime: 0,
            next: 0,
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
        let neighbors = get_neighbors_idxs(idx);
        let cell = world.grid[idx].to_cell().unwrap();
        cell.lifetime += 1;

        if cell.lifetime > 50 && cell.next != -1 {
            world.grid[idx] = Segment::Air;
            return;
        } else if cell.lifetime > 100 && cell.next == -1 {
            world.grid[idx] = Segment::Air;
            return;
        }

        if cell.is_seed {
            if let Segment::Air = world_read.grid[neighbors[BOTTOM]] {
                world.grid[neighbors[BOTTOM]] = Segment::Cell(cell.clone());
                world.grid[idx] = Segment::Air;
            } else if let Segment::Block(_) = world_read.grid[neighbors[BOTTOM]] {
                cell.next = 0;
                cell.lifetime = 0;
                world.grid[idx].to_cell().unwrap().is_seed = false;
            }
        } else {
            if !cell.is_seed {
                let mut i = 1;
                let mut children: Vec<Cell> = neighbors
                    .iter()
                    .filter(|_| cell.next != -1)
                    .map(|_| {
                        let res = Cell {
                            next: cell.genome.genes[cell.next as usize][i],
                            ..cell.clone()
                        };
                        i += 1;

                        res
                    })
                    .collect();

                for (idx, child) in children.iter_mut().enumerate() {
                    if child.next != 0 {
                        if let Segment::Air = world_read.grid[neighbors[idx]] {
                            if child.next == -1 {
                                child.is_seed = true;
                            }
                            world.grid[neighbors[idx]] = Segment::Cell(child.clone());
                        }
                    }
                }
            }
        }
    }
}

use rand::Rng;
use sdl2::rect::Rect;

use crate::{
    constants::{AMOUT_GENES, BOTTOM, SIZE_RECT_RENDER, SIZE_WORLD, TOP},
    traits::{Behaviour, Render},
    world::{get_neighbors_idxs, get_pos, World},
};

use super::{Air, Genome, Segment};

/// `Cell` is the main, executive essence of the simulation.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub id_family: usize,
    pub light_absorption_coefficient: f32,
    pub light: f32,
    pub energy: f32,
    pub is_seed: bool,
    pub lifetime: usize,
    pub next: i8,
    pub color: (u8, u8, u8),
    pub genome: Genome,
}

impl Cell {
    /// Creates a new cell with a given amount of energy.
    pub fn new(energy: f32) -> Self {
        Self {
            id_family: 0,
            light_absorption_coefficient: 0.5,
            light: 1.0,
            energy,
            is_seed: false,
            lifetime: 0,
            next: 0,
            color: (0x54, 0x92, 0x48),
            genome: Default::default(),
        }
    }

    pub fn mutate(&mut self) {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.05 {
            self.color = (
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
            );

            for gene in self.genome.genes.iter_mut() {
                for nucl in gene.iter_mut() {
                    *nucl = rand::thread_rng().gen_range(-1..(AMOUT_GENES as i8));
                }
            }
        }
    }

    pub fn generate_energy(&mut self, light: f32) {
        self.energy += light * 5.0;
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
            self.color
        });
        canvas.fill_rect(rect).unwrap();
    }
}

impl Behaviour for Cell {
    fn update(world_read: &World, world: &mut World, idx_segment: usize) {
        exchange_energy(world_read, world, idx_segment);

        let cell = world.grid[idx_segment].to_cell().unwrap();
        let neighbors = get_neighbors_idxs(idx_segment);

        if let Segment::Air(air) = &world_read.grid[neighbors[TOP]] {
            cell.light = air.light * cell.light_absorption_coefficient;
        } else if let Segment::Cell(n_cell) = &world_read.grid[neighbors[TOP]] {
            cell.light = n_cell.light * cell.light_absorption_coefficient;
        }

        cell.lifetime += 1;

        if cell.lifetime > 1500 && cell.next != -1 {
            world.grid[idx_segment] = Segment::Air(Air::default());
            return;
        } else if cell.lifetime > 2000 && cell.next == -1 {
            world.grid[idx_segment] = Segment::Air(Air::default());
            return;
        }

        if cell.energy < 20.0 {
            world.grid[idx_segment] = Segment::Air(Air::default());
            return;
        }

        if cell.is_seed {
            cell.energy -= 0.5;

            if let Segment::Air(_) = world_read.grid[neighbors[BOTTOM]] {
                world.grid[neighbors[BOTTOM]] = Segment::Cell(cell.clone());
                world.grid[idx_segment] = Segment::Air(Air::default());
            } else if let Segment::Block(_) = world_read.grid[neighbors[BOTTOM]] {
                cell.next = 0;
                cell.lifetime = 0;
                cell.id_family = rand::thread_rng().gen_range(0..10000);
                cell.mutate();
                world.grid[idx_segment].to_cell().unwrap().is_seed = false;
            }
        } else {
            if !cell.is_seed {
                cell.generate_energy(cell.light);
                cell.energy -= 1.0;
            }

            if !cell.is_seed && cell.energy >= 100.0 {
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
                    let cell = world.grid[idx_segment].to_cell().unwrap();

                    if child.next != 0 {
                        if let Segment::Air(_) = world_read.grid[neighbors[idx]] {
                            child.energy = 0.0;

                            if child.next == -1 {
                                child.energy = 100.0;
                                cell.energy -= 100.0;

                                child.is_seed = true;
                            } else {
                                child.energy = 25.0;
                                cell.energy -= 25.0;
                            }

                            world.grid[neighbors[idx]] = Segment::Cell(child.clone());
                        }
                    }
                }
            }
        }
    }
}

fn is_need_energy(from: &Cell, to: &Cell) -> bool {
    from.energy > to.energy && from.energy < 255.0
}

fn exchange_energy(world_read: &World, world: &mut World, idx_segment: usize) {
    let neighbors = get_neighbors_idxs(idx_segment);
    let count = 5.0;

    for i in 0..neighbors.len() {
        if let Segment::Cell(neighbor) = &world_read.grid[neighbors[i]] {
            let cell = world.grid[idx_segment].to_cell().unwrap();
            if neighbors[i] == idx_segment || cell.id_family != neighbor.id_family {
                continue;
            }

            if is_need_energy(world.grid[idx_segment].to_cell().unwrap(), neighbor) {
                if let Segment::Cell(_) = &world.grid[neighbors[i]] {
                    world.grid[idx_segment].to_cell().unwrap().energy -= count;
                    world.grid[neighbors[i]].to_cell().unwrap().energy += count;
                }
            }
        }
    }
}

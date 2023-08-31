use nalgebra::Vector2;

use crate::{
    constants::SIZE_WORLD,
    grid::{Block, Cell, Genome, Grid, Segment},
};

#[derive(Debug, Clone)]
pub struct World {
    pub light: f32,
    pub grid: Grid,
}

impl World {
    pub fn new() -> Self {
        Self {
            light: 1.0,
            grid: Default::default(),
        }
    }

    /// Generate the world with 1 living cell.
    pub fn generate(&mut self) {
        let mut grid = Grid::new(SIZE_WORLD[0] * SIZE_WORLD[1]);
        grid.segments.iter_mut().enumerate().for_each(|(i, seg)| {
            if i < SIZE_WORLD[0] {
                *seg = Segment::Block(Block::default());
            }
        });
        grid[get_index(128, 30, SIZE_WORLD[0])] = Segment::Cell(Cell {
            id_family: 0,
            light_absorption_coefficient: 0.5,
            light: 1.0,
            energy: 100.0,
            is_seed: true,
            lifetime: 0,
            color: (0x54, 0x92, 0x48),
            next: -1,
            genome: Genome::default(),
        });

        self.grid = grid;
    }

    pub fn get_segments_at(&self, idxs: Vec<usize>) -> Vec<(usize, Segment)> {
        idxs.iter()
            .filter(|i| **i < SIZE_WORLD[0] * SIZE_WORLD[1])
            .map(|i| (*i, self.grid[*i].clone()))
            .collect()
    }
}

/// Getting the segment index by its position.
pub const fn get_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Getting the segment position by its index.
pub const fn get_pos(index: usize, width: usize) -> Vector2<usize> {
    Vector2::new(index % width, index / width)
}

/// Get all neighboring segment segments.
pub fn get_neighbors_idxs(idx_segment: usize) -> [usize; 4] {
    let pos = get_pos(idx_segment, SIZE_WORLD[0]);
    let (w_max, h_max) = (SIZE_WORLD[0] as i32 - 1, SIZE_WORLD[1] as i32 - 1);
    let (x, y) = (pos.x as i32, pos.y as i32);
    let neighbors = [
        get_index(limit(w_max, x - 1) as usize, y as usize, SIZE_WORLD[0]),
        get_index(limit(w_max, x + 1) as usize, y as usize, SIZE_WORLD[0]),
        get_index(x as usize, limit(h_max, y + 1) as usize, SIZE_WORLD[0]),
        get_index(x as usize, limit(h_max, y - 1) as usize, SIZE_WORLD[0]),
    ];

    neighbors
}

fn limit(max: i32, n: i32) -> i32 {
    if n < 0 {
        return 0;
    } else if n > max {
        return max;
    }

    n
}

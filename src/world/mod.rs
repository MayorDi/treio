use nalgebra::Vector2;

use crate::{grid::{Grid, Segment, Cell, Genome, Block}, constants::SIZE_WORLD};

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
        grid[get_index(128, 32, SIZE_WORLD[0])] = Segment::Cell(Cell {
            energy: 100.0,
            is_seed: true,
            genome: Genome::default()
        });

        self.grid = grid;
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
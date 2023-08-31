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

/// Get all neighboring segment segments.
pub fn get_neighbors_idxs(idx_segment: usize) -> [usize; 4] {
    let pos = get_pos(idx_segment, SIZE_WORLD[0]);
    let (w_max, h_max) = (SIZE_WORLD[0] as i32, SIZE_WORLD[1] as i32);
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
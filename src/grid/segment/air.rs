use crate::{
    constants::TOP,
    traits::Behaviour,
    world::{get_neighbors_idxs, World},
};

use super::Segment;

#[derive(Default, Debug, Clone, Copy)]
pub struct Air {
    pub light: f32,
}

impl Behaviour for Air {
    fn update(world_read: &World, world: &mut World, idx_segment: usize) {
        if let Segment::Air(_) = world.grid[idx_segment] {
            let air = world.grid[idx_segment].to_air().unwrap();
            let neighbors = get_neighbors_idxs(idx_segment);

            if neighbors[TOP] == idx_segment {
                air.light = world.light;
            } else {
                if let Segment::Cell(cell) = &world_read.grid[neighbors[TOP]] {
                    air.light = cell.light;
                } else if let Segment::Air(n_air) = &world_read.grid[neighbors[TOP]] {
                    air.light = n_air.light;
                }
            }
        }
    }
}

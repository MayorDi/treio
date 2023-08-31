use crate::{grid::*, traits::Behaviour, world::World};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..world_read.grid.segments.len() {
            match &world_read.grid[i] {
                Segment::Block(_) => {
                    Block::update(world_read, &mut self.world, i);
                }

                Segment::Cell(_) => {
                    Cell::update(world_read, &mut self.world, i);
                }

                Segment::Air(_) => {
                    Air::update(world_read, &mut self.world, i);
                }
            }
        }
    }
}

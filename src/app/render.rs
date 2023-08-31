use crate::{grid::Segment, traits::Render, world::World};

use super::{App, SDL};

impl App {
    pub fn render(world_read: &World, sdl: &mut SDL) {
        for (idx, segment) in world_read.grid.segments.iter().enumerate() {
            match segment {
                Segment::Cell(cell) => cell.render(sdl, idx),
                Segment::Block(block) => block.render(sdl, idx),
                _ => {}
            }
        }
    }
}

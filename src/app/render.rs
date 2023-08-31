use crate::{grid::Segment, traits::Render, world::World};

use super::{App, SDL};

impl App {
    pub fn render(world_read: &World, sdl: &mut SDL) {
        for segment in world_read.grid.segments.iter() {
            match segment {
                Segment::Cell(cell) => cell.render(sdl),
                Segment::Block(block) => block.render(sdl),
                _ => {}
            }
        }
    }
}

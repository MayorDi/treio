mod block;
mod air;
mod cell;
mod genome;

pub use block::*;
pub use air::*;
pub use cell::*;
pub use genome::*;

#[derive(Debug, Clone)]
pub enum Segment {
    Air(Air),
    Cell(Cell),
    Block(Block),
}

impl Segment {
    pub fn to_cell(&mut self) -> Option<&mut Cell> {
        if let Segment::Cell(cell) = self {
            return Some(cell);
        }

        None
    }

    pub fn to_air(&mut self) -> Option<&mut Air> {
        if let Segment::Air(air) = self {
            return Some(air);
        }

        None
    }
}

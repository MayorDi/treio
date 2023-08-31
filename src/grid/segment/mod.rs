mod block;
mod cell;
mod genome;

pub use block::*;
pub use cell::*;
pub use genome::*;

#[derive(Debug, Clone, Default)]
pub enum Segment {
    #[default]
    Air,
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
}
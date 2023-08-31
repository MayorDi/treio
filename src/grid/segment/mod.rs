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

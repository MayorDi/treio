mod segment;

use std::ops::{Index, IndexMut};

pub use segment::*;

#[derive(Debug, Default, Clone)]
pub struct Grid {
    pub segments: Vec<Segment>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid {
            segments: vec![Segment::default(); size],
        }
    }
}

impl From<Vec<Segment>> for Grid {
    fn from(value: Vec<Segment>) -> Self {
        Grid { segments: value }
    }
}

impl Index<usize> for Grid {
    type Output = Segment;

    fn index(&self, index: usize) -> &Self::Output {
        &self.segments[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.segments[index]
    }
}

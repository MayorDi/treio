use crate::grid::Grid;

#[derive(Debug, Clone)]
pub struct World {
    pub grid: Grid,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: Default::default(),
        }
    }

    pub fn generate(&mut self) {}
}

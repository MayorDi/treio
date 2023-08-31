use rand::Rng;

use crate::constants::AMOUT_GENES;

/// `Genome` - contains all the necessary genes for the operation of the cell reproduction apparatus.
///
/// ## Feature:
/// * `-1` - a new flicker is a seed;
/// * `0` - the cell does not change and does not share.
#[derive(Debug, Clone, Copy)]
pub struct Genome {
    pub genes: [[i8; 5]; crate::constants::AMOUT_GENES],
}

impl Genome {
    pub fn new() -> Self {
        Self {
            genes: [[0; 5]; crate::constants::AMOUT_GENES],
        }
    }

    pub fn mutate(&mut self) {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.05 {
            for gene in self.genes.iter_mut() {
                for nucl in gene.iter_mut() {
                    *nucl = rand::thread_rng().gen_range(-1..(AMOUT_GENES as i8));
                }
            }
        }
    }
}

impl Default for Genome {
    fn default() -> Self {
        let mut genome = Genome::new();
        genome.genes[0] = [1, -1, -1, -1, 0];
        genome.genes[1] = [0; 5];

        genome
    }
}

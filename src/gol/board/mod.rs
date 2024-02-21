use crate::{
    countable::Countable,
    lattice::{tetra::Tetragonal2, var::LatticeVariant},
    types::Vec2i,
};

#[derive(Debug)]
pub(crate) struct Board {
    width: usize,
    height: usize,
    buf: Vec<bool>,
    lattice: LatticeVariant,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buf: (0..(width * height)).into_iter().map(|_| false).collect(),
            lattice: LatticeVariant::Tetra2(Tetragonal2::new()),
        }
    }
}

impl Countable for Board {
    type Item = Vec2i;

    fn count(&self, item: Self::Item) -> usize {
        item[0] as usize * &self.width + item[1] as usize
    }

    fn decount(&self, index: usize) -> Self::Item {
        [(index / self.width) as isize, (index % self.width) as isize]
    }
}

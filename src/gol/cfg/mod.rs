use crate::lattice::{tetra::Tetragonal2, var::LatticeVariant};

// Configuration for Game Mech
pub struct GolCfg {
    // width of the game
    width: usize,

    // height of the game
    height: usize,

    // lattice of the game
    lattice: LatticeVariant,
}

impl Default for GolCfg {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            lattice: LatticeVariant::Tetra2(Tetragonal2::new()),
        }
    }
}

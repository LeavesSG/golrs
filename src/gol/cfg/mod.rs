// Configuration for Game Mech
#[derive(Clone, Copy)]
pub struct GolCfg {
    // Width of the board
    pub width: usize,
    // Height of the board
    pub height: usize,
    // Length of the board, for 2d board, length will always be 1.
    pub length: usize,
    // Lattice used in the board.
    pub lattice: &'static str,
}

impl Default for GolCfg {
    fn default() -> Self {
        Self {
            width: 4,
            height: 4,
            length: 4,
            lattice: "Tetra2",
        }
    }
}

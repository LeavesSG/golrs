use std::fmt::Debug;

use super::{board::Board, cfg::GolCfg, rule::GolRule};

pub struct GameOfLife {
    board: Board,
    rule: GolRule,
    cfg: GolCfg,
}

impl GameOfLife {
    pub fn new(width: isize, height: isize) -> Self {
        GameOfLife {
            board: Board::new(width as usize, height as usize),
            rule: Default::default(),
            cfg: Default::default(),
        }
    }

    pub fn on_frame(&self) {}
}

impl Debug for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameOfLife")
            .field("board", &self.board)
            .finish()
    }
}

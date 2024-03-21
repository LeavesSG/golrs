use std::{fmt::Debug, str::FromStr};

use crate::lattice::variant::LatVar;

use super::{board::Board, cfg::GolCfg, mech::GameMech, rule::GolRule};

pub struct GameOfLife {
    pub board: Box<dyn GameMech>,
    pub rule: GolRule,
    pub cfg: GolCfg,
}

impl GameOfLife {
    pub fn new() -> Self {
        let cfg = GolCfg::default();
        let rule = GolRule::default();
        let board = Self::init_board(&cfg);
        GameOfLife { rule, cfg, board }
    }

    pub fn init_board(cfg: &GolCfg) -> Box<dyn GameMech> {
        let GolCfg {
            width,
            height,
            length,
            lattice,
        } = cfg;
        let lat_var = LatVar::from_str(lattice).unwrap_or(LatVar::default());
        match &lat_var {
            LatVar::Lat2(_) => Box::new(Board::<2>::new(
                [*width, *height],
                Box::new(lat_var.lat2().unwrap()),
            )),
            LatVar::Lat3(_) => Box::new(Board::<3>::new(
                [*width, *height, *length],
                Box::new(lat_var.lat3().unwrap()),
            )),
        }
    }
}

impl Default for GameOfLife {
    fn default() -> Self {
        let cfg: GolCfg = Default::default();
        let rule: GolRule = Default::default();
        let board = GameOfLife::init_board(&cfg);
        Self { rule, cfg, board }
    }
}

impl Debug for GameOfLife {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameOfLife")
            .field("board", &self.board.to_snapshot())
            .finish()
    }
}

use crate::countable::CountableVec;

use self::snapshot::{ConsumeSnapshot, NextSnapshot, ToSnapshot};
pub mod alive;
pub mod snapshot;

pub trait GameMech: ToSnapshot + ConsumeSnapshot + NextSnapshot + CountableVec {}

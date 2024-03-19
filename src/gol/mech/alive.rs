use crate::gol::rule::GolRule;

pub trait CellAlive {
    type Item;
    fn is_alive(&self, item: Self::Item) -> bool;
}

pub trait CellWillAlive: CellAlive {
    fn will_alive(&self, item: Self::Item, rule: &GolRule) -> bool;
}

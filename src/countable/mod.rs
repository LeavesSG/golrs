pub trait Countable {
    type Item;
    fn count(&self, item: Self::Item) -> usize;
    fn decount(&self, index: usize) -> Self::Item;
}

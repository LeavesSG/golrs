pub trait Countable {
    type Item: Sized;

    fn count(&self, item: Self::Item) -> usize;
    fn decount(&self, index: usize) -> Self::Item;
}

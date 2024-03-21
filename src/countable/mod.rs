pub trait Countable {
    type Item;
    fn count(&self, item: Self::Item) -> usize;
    fn decount(&self, index: usize) -> Self::Item;
}

pub trait CountableVec {
    fn count_vec(&self, item: Vec<isize>) -> usize;
    fn decount_vec(&self, index: usize) -> Vec<isize>;
}

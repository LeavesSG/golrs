pub trait Fold {
    type Item;
    fn fold(&self, item: Self::Item) -> Option<Self::Item>;
}

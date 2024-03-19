pub trait SharedVertex {
    type Item;
    fn shared_vertices(&'_ self, item: Self::Item) -> Vec<Self::Item>;
}

pub trait SharedEdge: SharedVertex {
    fn shared_edges(&'_ self, item: Self::Item) -> Vec<Self::Item>;
}

pub trait SharedFace: SharedEdge {
    fn shared_faces(&'_ self, item: Self::Item) -> Vec<Self::Item>;
}

pub trait TieredAdjacent {
    type Item;
    fn tiered_adj(&self, item: Self::Item) -> Vec<Vec<Self::Item>>;
}

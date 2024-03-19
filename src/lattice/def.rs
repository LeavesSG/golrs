use std::fmt::Debug;

use crate::types::{Vec2i, Vec3i, VecN};

use super::adjacent::{SharedEdge, SharedFace, SharedVertex};

pub trait BLattice<const D: usize>: Debug {
    fn image(&self, pre_image: VecN<f64, D>) -> VecN<f64, D>;
    fn pre_image(&self, image: VecN<f64, D>) -> VecN<f64, D>;
    fn shape_at(&self, index: VecN<isize, D>) -> Vec<VecN<f64, D>>;
}

impl SharedVertex for dyn BLattice<2> {
    type Item = Vec2i;
    fn shared_vertices(&self, index: Self::Item) -> Vec<Self::Item> {
        vec![
            [index[0] - 1, index[1] - 1],
            [index[0] + 1, index[1] - 1],
            [index[0] - 1, index[1] + 1],
            [index[0] + 1, index[1] + 1],
        ]
    }
}

impl SharedEdge for dyn BLattice<2> {
    fn shared_edges(&self, index: Self::Item) -> Vec<Self::Item> {
        vec![
            [index[0], index[1] - 1],
            [index[0] + 1, index[1]],
            [index[0], index[1] + 1],
            [index[0] - 1, index[1]],
        ]
    }
}

impl SharedVertex for dyn BLattice<3> {
    type Item = Vec3i;
    fn shared_vertices(&self, index: Self::Item) -> Vec<Self::Item> {
        vec![
            [index[0] - 1, index[1] - 1, index[2] - 1],
            [index[0] - 1, index[1] - 1, index[2] + 1],
            [index[0] - 1, index[1] + 1, index[2] + 1],
            [index[0] - 1, index[1] + 1, index[2] - 1],
            [index[0] + 1, index[1] - 1, index[2] - 1],
            [index[0] + 1, index[1] - 1, index[2] + 1],
            [index[0] + 1, index[1] + 1, index[2] + 1],
            [index[0] + 1, index[1] + 1, index[2] - 1],
        ]
    }
}

impl SharedEdge for dyn BLattice<3> {
    fn shared_edges(&self, index: Self::Item) -> Vec<Self::Item> {
        vec![
            [index[0], index[1] - 1, index[2] - 1],
            [index[0], index[1] - 1, index[2] + 1],
            [index[0], index[1] + 1, index[2] - 1],
            [index[0], index[1] + 1, index[2] + 1],
            [index[0] - 1, index[1], index[2] - 1],
            [index[0] - 1, index[1], index[2] + 1],
            [index[0] + 1, index[1], index[2] - 1],
            [index[0] + 1, index[1], index[2] + 1],
            [index[0] - 1, index[1] - 1, index[2] - 1],
            [index[0] - 1, index[1] + 1, index[2] + 1],
            [index[0] + 1, index[1] - 1, index[2] - 1],
            [index[0] + 1, index[1] + 1, index[2] + 1],
        ]
    }
}

impl SharedFace for dyn BLattice<3> {
    fn shared_faces(&self, index: Self::Item) -> Vec<Self::Item> {
        vec![
            [index[0], index[1], index[2] - 1],
            [index[0], index[1], index[2] + 1],
            [index[0], index[1] - 1, index[2]],
            [index[0], index[1] + 1, index[2]],
            [index[0] - 1, index[1], index[2]],
            [index[0] + 1, index[1], index[2]],
        ]
    }
}

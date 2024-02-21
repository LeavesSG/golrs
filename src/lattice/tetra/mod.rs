use super::def::BLattice;
use crate::types::VecN;

#[derive(Debug)]
pub struct Tetragonal2;

impl Tetragonal2 {
    pub fn new() -> Self {
        Tetragonal2 {}
    }
}

impl BLattice<2, 4> for Tetragonal2 {
    const BASE_SHAPE: [VecN<f64, 2usize>; 4] =
        [[0f64, 0f64], [1f64, 0f64], [1f64, 1f64], [0f64, 1f64]];

    fn image(&self, pre_image: VecN<f64, 2usize>) -> VecN<f64, 2usize> {
        pre_image
    }

    fn pre_image(&self, image: VecN<f64, 2usize>) -> VecN<f64, 2usize> {
        image
    }

    fn shape_at(&self, index: VecN<isize, 2usize>) -> [VecN<f64, 2usize>; 4] {
        let tr = self.image([index[0] as f64, index[1] as f64]);
        Self::BASE_SHAPE.map(|pos| [pos[0] + tr[0], pos[1] + tr[1]])
    }

    fn get_shared_vts(&self, index: [isize; 2]) -> Vec<[isize; 2]> {
        let [x, y] = index;
        vec![
            [x - 1, y - 1],
            [x, y - 1],
            [x + 1, y - 1],
            [x - 1, y],
            [x + 1, y],
            [x - 1, y + 1],
            [x, y + 1],
            [x + 1, y + 1],
        ]
    }
}

#[test]
fn test() {
    let t = Tetragonal2::new();
    let shape = t.shape_at([3, 2]);
    println!("{:?}", shape);
}

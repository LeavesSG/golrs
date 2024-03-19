use super::def::BLattice;
use crate::types::VecN;

#[derive(Debug, Clone)]
pub struct Tetragonal<const D: usize> {
    shape: Vec<VecN<f64, D>>,
}

impl<const D: usize> Tetragonal<D> {
    pub fn new(shape: Vec<VecN<f64, D>>) -> Self {
        Tetragonal { shape }
    }
}

impl Default for Tetragonal<2> {
    fn default() -> Self {
        Self {
            shape: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        }
    }
}

impl Default for Tetragonal<3> {
    fn default() -> Self {
        Self {
            shape: vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 1.0],
                [1.0, 1.0, 1.0],
                [1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

impl<const D: usize> BLattice<D> for Tetragonal<D> {
    fn image(&self, pre_image: VecN<f64, D>) -> VecN<f64, D> {
        pre_image
    }

    fn pre_image(&self, image: VecN<f64, D>) -> VecN<f64, D> {
        image
    }

    fn shape_at(&self, index: VecN<isize, D>) -> Vec<VecN<f64, D>> {
        let tr = self.image(index.map(|n| n as f64));
        self.shape
            .iter()
            .map(|pos| {
                let mut d = 0;
                pos.map(|val| {
                    let res = val + tr[d];
                    d += 1;
                    res
                })
            })
            .collect()
    }
}

#[test]
fn test() {
    let t = Tetragonal::<3>::default();
    let shape = t.shape_at([3, 2, 2]);
    println!("{:?}", shape);
}

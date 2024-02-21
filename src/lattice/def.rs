use crate::types::VecN;

pub trait BLattice<const Dim: usize, const S: usize> {
    const BASE_SHAPE: [VecN<f64, Dim>; S];
    fn image(&self, pre_image: VecN<f64, Dim>) -> VecN<f64, Dim>;
    fn pre_image(&self, image: VecN<f64, Dim>) -> VecN<f64, Dim>;

    fn shape_at(&self, index: VecN<isize, Dim>) -> [VecN<f64, Dim>; S];
    fn get_shared_vts(&self, index: VecN<isize, Dim>) -> Vec<[isize; Dim]>;
}

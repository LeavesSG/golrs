pub type VecN<T, const N: usize> = [T; N];

pub type Vec2f = VecN<f64, 2>;
pub type Vec2i = VecN<isize, 2>;
pub type Vec2u = VecN<usize, 2>;

pub type Vec3f = VecN<f64, 3>;
pub type Vec3i = VecN<isize, 3>;
pub type Vec3u = VecN<usize, 3>;

use std::{fmt::Debug, ops::Index, usize};

use crate::{
    countable::{Countable, CountableVec},
    lattice::{adjacent::*, def::BLattice, fold::Fold},
    types::{Vec2i, Vec2u, Vec3i, Vec3u, VecN},
};

use super::{
    mech::{
        alive::{CellAlive, CellWillAlive},
        snapshot::*,
        GameMech,
    },
    rule::GolRule,
};

#[derive(Debug)]
pub struct Board<const D: usize> {
    size: VecN<usize, D>,
    buf: Vec<bool>,
    lattice: Box<dyn BLattice<D>>,
}

impl Board<2> {
    pub fn new(size: Vec2u, lattice: Box<dyn BLattice<2>>) -> Self {
        let len = size.iter().product::<usize>();
        Self {
            size,
            buf: (0..len).map(|_| false).collect(),
            lattice,
        }
    }
}

impl Board<3> {
    pub fn new(size: Vec3u, lattice: Box<dyn BLattice<3>>) -> Self {
        let len = size.iter().product::<usize>();
        Self {
            size,
            buf: (0..len).map(|_| false).collect(),
            lattice,
        }
    }
}

impl<const D: usize> Index<usize> for Board<D> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        self.buf.index(index)
    }
}

impl<const D: usize> Board<D> {
    fn product(&self, n: usize) -> usize {
        self.size.iter().take(n).product()
    }
}

impl<const D: usize> Fold for Board<D> {
    type Item = VecN<isize, D>;
    fn fold(&self, item: VecN<isize, D>) -> Option<VecN<isize, D>> {
        let mut target: [isize; D] = item;
        target.iter_mut().enumerate().for_each(|(index, value)| {
            let bound = self.size[index] as isize;
            if *value < 0 {
                *value = *value % bound + bound
            } else {
                *value %= bound
            }
        });
        Some(target)
    }
}

impl<const D: usize> Countable for Board<D> {
    type Item = VecN<isize, D>;
    fn count(&self, item: Self::Item) -> usize {
        item.into_iter()
            .enumerate()
            .rev()
            .fold(0, |acc, (dim, val)| acc + val as usize * self.product(dim))
    }

    fn decount(&self, index: usize) -> Self::Item {
        let mut item = self.size.map(|v| v as isize);
        let mut dim = item.len() as isize - 1;
        let mut index = index as isize;
        while dim >= 0 {
            let product = self.product(dim as usize) as isize;
            item[dim as usize] = index / product;
            index -= item[dim as usize] * product;
            dim -= 1;
        }
        item
    }
}

impl<const D: usize> CountableVec for Board<D> {
    fn count_vec(&self, item: Vec<isize>) -> usize {
        let mut arr = self.decount(0);
        let len = arr.len();
        arr.copy_from_slice(&item[..len]);
        self.count(arr)
    }

    fn decount_vec(&self, index: usize) -> Vec<isize> {
        self.decount(index).into()
    }
}

impl<T, I> CellAlive for T
where
    T: Index<usize, Output = bool> + Countable<Item = I>,
{
    type Item = I;
    fn is_alive(&self, item: Self::Item) -> bool {
        let index = self.count(item);
        *self.index(index)
    }
}

impl TieredAdjacent for Board<2> {
    type Item = Vec2i;
    fn tiered_adj(&self, item: Self::Item) -> Vec<Vec<Self::Item>> {
        vec![
            self.lattice.shared_vertices(item),
            self.lattice.shared_edges(item),
        ]
    }
}

impl TieredAdjacent for Board<3> {
    type Item = Vec3i;
    fn tiered_adj(&self, item: Self::Item) -> Vec<Vec<Self::Item>> {
        vec![
            self.lattice.shared_vertices(item),
            self.lattice.shared_edges(item),
            self.lattice.shared_faces(item),
        ]
    }
}

impl<T, I: Copy> CellWillAlive for T
where
    T: Fold<Item = I> + CellAlive<Item = I> + TieredAdjacent<Item = I>,
{
    fn will_alive(&self, item: Self::Item, rule: &GolRule) -> bool {
        let tiered_adj = self.tiered_adj(item);
        let tier = tiered_adj.len();
        let alive_rates = tiered_adj.into_iter().map(|adj| {
            let valid = adj.iter().filter_map(|&item| self.fold(item));
            let mut count = 0.;
            let alive = valid
                .filter(|&item| {
                    count += 1.;
                    self.is_alive(item)
                })
                .count() as f64;
            alive / count
        });
        let normalized_factors = rule.normalized_tier_factors(tier);
        let calc_rate: f64 = alive_rates.enumerate().fold(0., |acc, (index, rate)| {
            let factor = if index < normalized_factors.len() {
                normalized_factors[index]
            } else {
                0.
            };
            acc + factor * rate
        });
        let threshold = match self.is_alive(item) {
            true => rule.alive_threshold,
            false => rule.dead_threshold,
        };
        calc_rate >= threshold.0 && calc_rate < threshold.1
    }
}

impl<const D: usize> BufSize for Board<D> {
    fn buf_size(&self) -> usize {
        self.size.iter().product()
    }
}

impl<const D: usize> ToSnapshot for Board<D> {
    fn to_snapshot(&self) -> Snapshot {
        Snapshot::new(self.buf.clone())
    }
}

impl<const D: usize> ConsumeSnapshot for Board<D> {
    fn consume_snapshot(&mut self, snapshot: Snapshot) {
        self.buf = snapshot.consume();
    }
}

impl<T, I> NextSnapshot for T
where
    T: CellAlive<Item = I>
        + CellWillAlive<Item = I>
        + Countable<Item = I>
        + ConsumeSnapshot
        + ToSnapshot,
{
    fn next_snapshot(&self, rule: &GolRule) -> Snapshot {
        let mut snapshot = self.to_snapshot();
        for index in 0..snapshot.buf.len() {
            let item = self.decount(index);
            snapshot.buf[index] = self.will_alive(item, rule);
        }
        snapshot
    }
}

impl<T, I> GameMech for T where
    T: CellAlive<Item = I>
        + CellWillAlive<Item = I>
        + Countable<Item = I>
        + ConsumeSnapshot
        + ToSnapshot
        + CountableVec
{
}

#[test]
fn test() {
    let rule = GolRule::default();
    let tetra3 = crate::lattice::variant::Lat2::default().lat2();
    let mut board = Board::<2>::new([4, 4], Box::new(tetra3));
    let snapshot = Snapshot::from_str(
        String::from(
            "
        0000
        0100
        0100
        0000",
        ),
        ('1', '0'),
    );
    board.consume_snapshot(snapshot.clone());
    let res = board.will_alive([1, 1], &rule);
    println!("{}", res);
}

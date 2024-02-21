use super::game::GameOfLife;

pub trait GolMech {
    type Coord: ?Sized;
    type Snapshot;

    fn is_alive(&self, index: &Self::Coord) -> bool;

    fn to_snapshot(&self) -> Self::Snapshot;

    fn from_snapshot(&self, state: Self::Snapshot);

    fn next_snapshot(&self) -> Self::Snapshot;
}

impl GolMech for GameOfLife {
    type Coord = [isize];

    type Snapshot = Vec<bool>;

    fn is_alive(&self, index: &Self::Coord) -> bool {
        todo!()
    }

    fn to_snapshot(&self) -> Self::Snapshot {
        todo!()
    }

    fn from_snapshot(&self, state: Self::Snapshot) {
        todo!()
    }

    fn next_snapshot(&self) -> Self::Snapshot {
        todo!()
    }
}

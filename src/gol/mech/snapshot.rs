use crate::gol::rule::GolRule;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Snapshot {
    pub buf: Vec<bool>,
}

impl Snapshot {
    pub fn new(buf: Vec<bool>) -> Self {
        Snapshot { buf }
    }

    pub fn consume(self) -> Vec<bool> {
        self.buf
    }

    fn append_from_u8(&mut self, number: u8) {
        for i in (0..8).rev() {
            self.buf.push((number >> i) % 2 == 1)
        }
    }

    pub fn from_u8(in_buf: Vec<u8>) -> Self {
        let mut snapshot = Snapshot::new(vec![]);
        for num in in_buf {
            snapshot.append_from_u8(num);
        }
        snapshot
    }
}

pub trait ToSnapshot {
    fn to_snapshot(&self) -> Snapshot;
}

pub trait ConsumeSnapshot {
    fn consume_snapshot(&mut self, snapshot: Snapshot);
}

pub trait NextSnapshot {
    fn next_snapshot(&self, rule: &GolRule) -> Snapshot;
}
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Debug, Default)]
pub struct GCounter {
    replica_id: u64,
    counts: BTreeMap<u64, u64>,
}

impl GCounter {
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let replica_id = NEXT.fetch_add(1, Ordering::Relaxed);

        Self {
            replica_id,
            counts: BTreeMap::new(),
        }
    }

    pub fn value(&self) -> u64 {
        self.counts.values().copied().sum()
    }

    pub fn inc(&mut self, n: u64) {
        *self.counts.entry(self.replica_id).or_insert(0) += n;
    }

    pub fn merge(&mut self, other: Self) {
        for (rid, other_count) in other.counts {
            self.counts
                .entry(rid)
                .and_modify(|c| *c = (*c).max(other_count))
                .or_insert(other_count);
        }
    }
}

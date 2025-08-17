use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::state::ReplicaId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VectorClock(pub BTreeMap<ReplicaId, u64>);

impl VectorClock {
    pub fn increment(&mut self, r: ReplicaId) {
        *self.0.entry(r).or_insert(0) += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (rid, counter) in &other.0 {
            self.0
                .entry(*rid)
                .and_modify(|c| *c = (*c).max(*counter))
                .or_insert(*counter);
        }
    }

    pub fn partial_cmp(&self, other: &VectorClock) -> VectorOrder {
        let mut le = true;
        let mut ge = true;
        let keys: BTreeSet<_> = self.0.keys().chain(other.0.keys()).cloned().collect();
        for k in keys {
            let a = self.0.get(&k).copied().unwrap_or(0);
            let b = other.0.get(&k).copied().unwrap_or(0);
            if a < b {
                ge = false;
            }
            if a > b {
                le = false;
            }
        }
        match (le, ge) {
            (true, true) => VectorOrder::Equal,
            (true, false) => VectorOrder::Before,
            (false, true) => VectorOrder::After,
            (false, false) => VectorOrder::Concurrent,
        }
    }

    pub fn dominates_or_equal(&self, other: &VectorClock) -> bool {
        matches!(
            other.partial_cmp(self),
            VectorOrder::After | VectorOrder::Equal
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorOrder {
    Before,
    After,
    Concurrent,
    Equal,
}

use std::collections::BTreeMap;

use crate::{
    metadata_storage::MetadataStorage,
    state::{
        ChangeEvent, ChangeId, ChangeOp, MaterializedState, ReplicaId, VectorClock, VectorOrder,
        sort_changes,
    },
};

/// A Repository holds the current state and change history.
///
/// This is the core of the timeline, managing changes and the current state.
pub struct Repository<S: MetadataStorage> {
    storage: S,
    replica: ReplicaId,
    current_clock: VectorClock,
    changes: BTreeMap<ChangeId, ChangeEvent>,
    state: MaterializedState,
}

impl<S: MetadataStorage> Repository<S> {
    /// Bootstraps a new repository from storage.
    pub fn bootstrap(storage: S) -> anyhow::Result<Self> {
        let changes = storage.load_all_changes()?;

        let mut map = BTreeMap::new();
        for c in changes {
            map.insert(c.id, c);
        }

        let mut ordered: Vec<ChangeEvent> = map.values().cloned().collect();
        sort_changes(&mut ordered);

        let replica = ordered.last().map_or(ReplicaId::new(), |c| c.replica);

        let state = MaterializedState::from_changes(&ordered);

        let mut current_clock = VectorClock::default();
        for c in ordered {
            current_clock.merge(&c.clock);
        }

        // let replica = current_clock.replica().unwrap_or_else(|| ReplicaId::new());

        Ok(Self {
            storage,
            replica,
            current_clock,
            changes: map,
            state,
        })
    }

    /// Applies a change the the repository.
    // TODO: does this deduplicate changes?
    pub fn apply_change(&mut self, op: ChangeOp) -> anyhow::Result<ChangeId> {
        self.current_clock.increment(self.replica);

        let change = ChangeEvent::new(op, self.replica, self.current_clock.clone());
        self.storage.append_change(&change)?;
        self.changes.insert(change.id, change.clone());
        self.integrate_change(change)?;

        Ok(self.state.head.unwrap())
    }

    /// Integrates a single change into the repositories state.
    ///
    /// Use [apply_local_change](Self::apply_local_change) to apply changes.
    /// This method only changes the materialized state and does not store the change.
    fn integrate_change(&mut self, change: ChangeEvent) -> anyhow::Result<()> {
        if self.changes.contains_key(&change.id) {
            return Ok(());
        }

        self.changes.insert(change.id, change.clone());
        self.current_clock.merge(&change.clock);

        self.apply_change(change.op)?;
        Ok(())
    }

    /// Returns the materialized state at the time of the given change ID.
    pub fn checkout(&self, at: ChangeId) -> MaterializedState {
        let mut subset: Vec<ChangeEvent> = self.changes.values().cloned().collect();
        sort_changes(&mut subset);

        for id in subset.clone().iter().map(|c| c.id) {
            subset.push(self.changes.get(&id).unwrap().clone());
            if id == at {
                break;
            }
        }

        MaterializedState::from_changes(&subset)
    }

    /// Returns the materialized state at the time of the given vector clock.
    pub fn checkout_clock(&self, target: &VectorClock) -> MaterializedState {
        let mut subset: Vec<ChangeEvent> = self
            .changes
            .values()
            .filter(|c| {
                matches!(
                    c.clock.partial_cmp(target),
                    VectorOrder::Before | VectorOrder::Equal
                )
            })
            .cloned()
            .collect();
        sort_changes(&mut subset);

        MaterializedState::from_changes(&subset)
    }

    pub fn get_change(&self, id: &ChangeId) -> Option<&ChangeEvent> {
        self.changes.get(id)
    }
    pub fn get_current_clock(&self) -> &VectorClock {
        &self.current_clock
    }
    pub fn get_state(&self) -> &MaterializedState {
        &self.state
    }
    pub fn get_changes(&self) -> &BTreeMap<ChangeId, ChangeEvent> {
        &self.changes
    }
    pub fn get_replica(&self) -> ReplicaId {
        self.replica
    }
}

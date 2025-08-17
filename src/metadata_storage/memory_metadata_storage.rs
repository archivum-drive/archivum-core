use crate::metadata_storage::MetadataStorage;
use crate::state::{ChangeEvent, ChangeId};
use std::collections::BTreeMap;

/// In-memory implementation of [MetadataStorage].
pub struct MemoryMetadataStorage {
    changes: BTreeMap<ChangeId, ChangeEvent>,
}

impl MemoryMetadataStorage {
    pub fn new() -> Self {
        Self {
            changes: BTreeMap::new(),
        }
    }
}
impl Default for MemoryMetadataStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataStorage for MemoryMetadataStorage {
    fn append_change(&mut self, change: &ChangeEvent) -> anyhow::Result<()> {
        self.changes.insert(change.id, change.clone());
        Ok(())
    }

    fn load_all_changes(&self) -> anyhow::Result<Vec<ChangeEvent>> {
        Ok(self.changes.values().cloned().collect())
    }

    fn get_change(&self, id: &ChangeId) -> anyhow::Result<Option<ChangeEvent>> {
        Ok(self.changes.get(id).cloned())
    }
}

use crate::state::{ChangeEvent, ChangeId};
pub use memory_metadata_storage::MemoryMetadataStorage;

mod memory_metadata_storage;

/// Local storage of current state and change events.
///
/// Interface for storing and retrieving metadata and change events.
/// Can be persistent or ephemeral since the [NetworkSync](crate::network::NetworkSync) trait handles synchronization.
pub trait MetadataStorage {
    /// Store a new ChangeEvent (append-only).
    fn append_change(&mut self, change: &ChangeEvent) -> anyhow::Result<()>;

    /// Retrieve all stored changes.
    ///
    /// Order is not important
    fn load_all_changes(&self) -> anyhow::Result<Vec<ChangeEvent>>;

    /// Retrieve a specific ChangeEvent by ID.
    fn get_change(&self, id: &ChangeId) -> anyhow::Result<Option<ChangeEvent>>;
}

use crate::state::ChangeEvent;

/// Sync changes over a network.
///
/// Used to push changes to remote for synchronization.
/// And pull changes that have been made by other clients.
pub trait NetworkSync {
    /// Push changes to remote.
    fn push_changes(&mut self, changes: Vec<ChangeEvent>) -> anyhow::Result<()>;

    /// Pull all changes that ever happend from remote.
    fn pull_changes(&mut self) -> anyhow::Result<Vec<ChangeEvent>>;
}

use std::{cell::RefCell, rc::Rc};

use crate::{
    metadata_storage::MetadataStorage,
    network::NetworkSync,
    node::{NodeId, NodeType},
    state::{ChangeEvent, ChangeOp, ReplicaId, Repository},
};

/// A client that handles local state and network synchronization.
pub struct Client<S: MetadataStorage, N: NetworkSync> {
    replica: ReplicaId,
    name: String,
    repo: Repository<S>,
    net_sync: Rc<RefCell<N>>,
}

impl<S: MetadataStorage, N: NetworkSync> Client<S, N> {
    pub fn new(name: &str, storage: S, net_sync: Rc<RefCell<N>>) -> anyhow::Result<Self> {
        let repo = Repository::bootstrap(storage)?;
        Ok(Self {
            replica: repo.get_replica(),
            name: name.into(),
            repo,
            net_sync,
        })
    }

    pub fn push(&mut self) -> anyhow::Result<()> {
        let all_changes: Vec<ChangeEvent> = self.repo.get_changes().values().cloned().collect();
        self.net_sync.borrow_mut().push_changes(all_changes)
    }
    pub fn pull(&mut self) -> anyhow::Result<()> {
        let remote = self.net_sync.borrow_mut().pull_changes()?;
        for c in remote {
            self.repo.apply_change(c.op)?;
        }
        Ok(())
    }

    pub fn create_file(&mut self, label: &str) -> anyhow::Result<NodeId> {
        let id = NodeId::new();
        self.repo.apply_change(ChangeOp::CreateNode {
            id,
            name: label.into(),
            node_type: NodeType::File,
        })?;
        Ok(id)
    }
    pub fn rename(&mut self, id: NodeId, new_name: &str) -> anyhow::Result<()> {
        self.repo.apply_change(ChangeOp::RenameNode {
            id,
            new_name: new_name.into(),
        })?;
        Ok(())
    }
    pub fn delete(&mut self, id: NodeId) -> anyhow::Result<()> {
        self.repo.apply_change(ChangeOp::DeleteNode { id })?;
        Ok(())
    }

    pub fn get_replica(&self) -> ReplicaId {
        self.replica
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_repo(&self) -> &Repository<S> {
        &self.repo
    }
    pub fn get_repo_mut(&mut self) -> &mut Repository<S> {
        &mut self.repo
    }
}

mod repository;
mod vector_clock;

pub use repository::*;
pub use vector_clock::*;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::node::{Node, NodeId, NodeType};

/// Materialized state of current nodes/blobs.
#[derive(Debug, Clone, Default)]
pub struct MaterializedState {
    pub nodes: BTreeMap<NodeId, Node>,
    pub head: Option<ChangeId>,
}

impl MaterializedState {
    /// Turns a list of [ChangeEvents](ChangeEvent) into a materialized state.
    pub fn from_changes(changes: &[ChangeEvent]) -> MaterializedState {
        let mut ordered: Vec<ChangeEvent> = changes.to_vec();
        sort_changes(&mut ordered);

        let mut state = MaterializedState::default();
        let mut last_id = None;

        for ch in &ordered {
            last_id = Some(ch.id);
            state.apply_change(ch);
        }
        state.head = last_id;
        state
    }

    /// Applies a single [ChangeEvent] to the materialized state.
    fn apply_change(&mut self, change: &ChangeEvent) {
        match &change.op {
            ChangeOp::CreateNode {
                id,
                name,
                node_type,
            } => {
                self.nodes
                    .entry(*id)
                    .or_insert_with(|| Node::new(*id, name.clone(), node_type.clone()));
            }
            ChangeOp::DeleteNode { id } => {
                self.nodes.remove(id);
            }
            ChangeOp::RenameNode { id, new_name } => {
                let node = self
                    .nodes
                    .get_mut(id)
                    .expect("Node with ID {id} not found for rename");

                node.set_name(new_name.clone());
            }
        }
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }
    pub fn list_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }
}

/// ID of a [ChangeEvent].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ChangeId(Uuid);

impl ChangeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for ChangeId {
    fn default() -> Self {
        Self::new()
    }
}

/// ID of a exact point in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ReplicaId(Uuid);
impl ReplicaId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for ReplicaId {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an operation on the timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeOp {
    // TODO: switch from variables to Node
    CreateNode {
        id: NodeId,
        name: String,
        node_type: NodeType,
    },
    DeleteNode {
        id: NodeId,
    },
    RenameNode {
        id: NodeId,
        new_name: String,
    },
    // TODO: AddMember, RemoveMember to be added
}

/// Represents an event with [ChangeOp] and its time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChangeEvent {
    pub id: ChangeId,
    pub replica: ReplicaId,
    pub clock: VectorClock,
    pub op: ChangeOp,
}

impl ChangeEvent {
    pub fn new(op: ChangeOp, replica: ReplicaId, clock: VectorClock) -> Self {
        Self {
            id: ChangeId::new(),
            replica,
            clock,
            op,
        }
    }
}

fn sort_changes(changes: &mut [ChangeEvent]) {
    changes.sort_by(|a, b| match a.clock.partial_cmp(&b.clock) {
        VectorOrder::Before => std::cmp::Ordering::Less,
        VectorOrder::After => std::cmp::Ordering::Greater,
        VectorOrder::Equal => a.id.cmp(&b.id),
        VectorOrder::Concurrent => a.id.cmp(&b.id),
    });
}

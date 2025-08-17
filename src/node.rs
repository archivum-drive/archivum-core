use std::{cmp::Ord, fmt::Display};

use crdts::merkle_reg::MerkleReg;
use uuid::Uuid;

use crate::blob::BlobId;

/// A stable, unique identifier for a Node.
///
/// This never changes, even if the Node is updated.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, PartialOrd, Ord,
)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Creates a new, random NodeId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}
impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The type of Node.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
pub enum NodeType {
    /// A single file.
    File,
    /// A directory-like group of Nodes.
    Group,
}

/// A node represents a single logical entity in the system,
/// like a file or a group etc.
///
/// It has a stable ID, a name, a type and either a reference to a Blob (for files)
/// or a set of member NodeIds (for groups).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Hash, Eq)]
pub struct Node {
    /// The stable, unique ID of this node.
    id: NodeId,

    /// A user-friendly name.
    name: String,

    /// A reference to the data blob.
    blob_ref: Option<BlobId>,

    /// The type of this node.
    node_type: NodeType,

    /// Set of NodeIds that are members of this node.
    members: MerkleReg<NodeId>,
}

impl Node {
    pub fn new(id: NodeId, name: String, node_type: NodeType) -> Self {
        Self {
            id,
            name,
            blob_ref: None,
            node_type,
            members: MerkleReg::new(),
        }
    }

    pub fn get_id(&self) -> &NodeId {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_blob_ref(&self) -> Option<&BlobId> {
        self.blob_ref.as_ref()
    }

    pub fn get_node_type(&self) -> &NodeType {
        &self.node_type
    }

    pub fn get_members(&self) -> &MerkleReg<NodeId> {
        &self.members
    }
}

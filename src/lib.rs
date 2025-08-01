use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// A SHA-256 hash of a raw, encrypted data blob. Immutable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BlobId(String);

/// A stable, unique identifier for a metadata Node. This never changes, even if the
/// Node's metadata is updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

/// The type of content a Node represents. This helps the UI render it correctly.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    /// A generic file.
    File,
    /// A directory-like structure. The actual members are stored in the `members` field.
    Group,
    // /// A bookmark to a URL.
    // Bookmark,
}

/// A node represents a single logical entity in the system,
/// like a file or a group etc.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Node {
    /// The stable, unique ID of this node.
    pub id: NodeId,

    /// A user-friendly name.
    pub name: String,

    /// A reference to the immutable, encrypted data blob.
    /// This is optional because a Group might not have its own content blob
    pub blob_ref: Option<BlobId>,

    /// The type of this node.
    pub node_type: NodeType,

    /// to be implemented
    pub attributes: HashMap<String, String>,

    /// Set of NodeIds that are members of this node.
    /// For example, a Group node will have its members listed here.
    pub members: HashSet<NodeId>,
}

pub struct Blob {
    /// The immutable hash of the blob.
    pub id: BlobId,

    /// Size of the blob in bytes.
    pub size: u64,
}

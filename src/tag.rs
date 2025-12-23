use std::{ collections::HashMap, fmt::Debug };

use getset::Getters;
use roaring::RoaringBitmap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TagId(pub u32);

impl From<u32> for TagId {
    fn from(value: u32) -> Self {
        TagId(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TagColors {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Gray,
}

#[derive(Clone, Debug, Getters, serde::Serialize, serde::Deserialize)]
#[getset(get = "pub with_prefix")]
pub struct TagRecord {
    id: TagId,

    pub(crate) deleted: bool,

    path: Vec<String>,

    color: TagColors,
}

impl TagRecord {
    pub fn new(id: TagId, path: Vec<String>, color: Option<TagColors>) -> Self {
        Self {
            id,
            deleted: false,
            path,
            color: color.unwrap_or(TagColors::Gray),
        }
    }
}

/// Derived hierarchy indexes (rebuildable from tags[*].path).
#[derive(Clone, Debug, Default)]
pub struct TagHierarchyIndex {
    pub parent: HashMap<TagId, Option<TagId>>,

    pub children: HashMap<TagId, Vec<TagId>>,
}

/// Derived tag lookup index (rebuildable from tags[*].path).
#[derive(Clone, Debug, Default)]
pub struct TagPathIndex {
    pub by_path: HashMap<String, TagId>,
}

/// Derived membership indexes (rebuildable from nodes[*].tags and hierarchy).
#[derive(Clone, Debug, Default)]
pub struct TagMembershipIndex {
    /// Nodes that explicitly contain this tag.
    pub direct_nodes: HashMap<TagId, RoaringBitmap>,

    /// Nodes that contain this tag OR any descendant tag.
    pub subtree_nodes: HashMap<TagId, RoaringBitmap>,
}

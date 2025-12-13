use std::collections::HashMap;

use crate::{
    node::{NodeBitmapIndex, NodeId, NodeRecord},
    tag::{TagHierarchyIndex, TagId, TagMembershipIndex, TagPathIndex, TagRecord},
};

#[derive(Clone, Debug, Default)]
pub struct Repo {
    pub nodes: HashMap<NodeId, NodeRecord>,
    pub tags: HashMap<TagId, TagRecord>,

    // rebuildable indexes
    pub node_index: NodeBitmapIndex,
    pub tag_paths: TagPathIndex,
    pub tag_hierarchy: TagHierarchyIndex,
    pub tag_membership: TagMembershipIndex,
}

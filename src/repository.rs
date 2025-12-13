use std::collections::HashMap;

use roaring::RoaringBitmap;

use crate::{
    node::{NodeBitmapIndex, NodeId, NodeRecord},
    tag::{TagColors, TagHierarchyIndex, TagId, TagMembershipIndex, TagPathIndex, TagRecord},
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

impl Repo {
    // ----------------------------
    // Construction / persistence
    // ----------------------------

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_bytes(_bytes: &[u8]) -> Result<Self, RepoError> {
        todo!("deserialize canonical state; rebuild_all_indexes");
    }

    pub fn save_to_bytes(&self) -> Result<Vec<u8>, RepoError> {
        todo!("serialize canonical state only");
    }

    // ----------------------------
    // Index rebuild / maintenance
    // ----------------------------

    pub fn rebuild_all_indexes(&mut self) {
        todo!("rebuild node_index, tag_paths, tag_hierarchy, tag_membership");
    }

    pub fn rebuild_node_index(&mut self) {
        todo!("assign NodeIx for each node; handle deleted nodes per your policy");
    }

    pub fn rebuild_tag_path_index_and_merge_duplicates(&mut self) -> Result<(), RepoError> {
        todo!("normalize paths; merge TagIds that share a path; update node references");
    }

    pub fn rebuild_tag_hierarchy_from_paths(&mut self) {
        todo!("derive parent prefix tag; build adjacency lists");
    }

    pub fn rebuild_tag_membership_indexes(&mut self) {
        todo!("direct_nodes from nodes[*].explicit_tags; subtree_nodes via hierarchy closure");
    }

    // ----------------------------
    // Tag operations
    // ----------------------------

    pub fn get_tag_by_path(&mut self, _path: Vec<&str>) -> Result<TagId, RepoError> {
        todo!("normalize path");
    }

    pub fn get_tag(&self, _tag: TagId) -> Option<&TagRecord> {
        todo!()
    }

    pub fn set_tag_path(&mut self, _tag: TagId, _new_path: Vec<&str>) -> Result<(), RepoError> {
        todo!(
            "update tag.path; dedup by path; then rebuild affected indexes (likely hierarchy + subtree)"
        );
    }

    pub fn create_tag(&mut self, _path: Vec<&str>, _color: TagColors) -> Result<TagId, RepoError> {
        todo!("normalize path; dedupe by path; insert new tag; rebuild indexes as needed");
    }

    pub fn delete_tag(&mut self, _tag: TagId) -> Result<(), RepoError> {
        todo!("tombstone tag; remove from nodes; rebuild membership");
    }

    // ----------------------------
    // Node operations
    // ----------------------------

    pub fn upsert_node(&mut self, _node: NodeRecord) -> Result<(), RepoError> {
        todo!("insert/update canonical node; update indexes incrementally or rebuild");
    }

    pub fn delete_node(&mut self, _node: NodeId) -> Result<(), RepoError> {
        todo!("mark deleted; update membership bitmaps");
    }

    pub fn get_node(&self, _node: NodeId) -> Option<&NodeRecord> {
        todo!()
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &NodeRecord> {
        self.nodes.values()
    }

    // ----------------------------
    // Tagging operations
    // ----------------------------

    pub fn add_tag(&mut self, _node: NodeId, _tag: TagId) -> Result<(), RepoError> {
        todo!("update node.explicit_tags; update membership indexes");
    }

    pub fn remove_tag(&mut self, _node: NodeId, _tag: TagId) -> Result<(), RepoError> {
        todo!("update node.explicit_tags; update membership indexes");
    }

    // ----------------------------
    // Queries
    // ----------------------------

    pub fn get_nodes_with_tag(&self, _tag: TagId) -> Option<&RoaringBitmap> {
        todo!("return subtree bitmap");
    }

    pub fn search_bitmap(&self, _query: TagQuery) -> Result<RoaringBitmap, RepoError> {
        todo!("combine subtree bitmaps with union/intersection/difference");
    }

    pub fn node_ids_from_bitmap<'a>(
        &'a self,
        _bm: &'a RoaringBitmap,
    ) -> impl Iterator<Item = NodeId> + 'a {
        todo!("map NodeIx -> NodeId; skip deleted nodes as desired");
        #[allow(unreachable_code)]
        std::iter::empty::<NodeId>()
    }
}

#[derive(Clone, Debug)]
pub enum TagQuery {
    Tag(TagId),
    Or(Box<TagQuery>, Box<TagQuery>),
    And(Box<TagQuery>, Box<TagQuery>),
    Not(Box<TagQuery>),
}

#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("not found")]
    NotFound,
    #[error("invalid path")]
    InvalidTagPath,
    #[error("serialization error")]
    Serialization,
    #[error("other: {0}")]
    Other(String),
}

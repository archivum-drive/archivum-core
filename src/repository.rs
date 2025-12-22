use std::collections::HashMap;

use roaring::RoaringBitmap;
use serde::{ Deserialize, Serialize, ser::SerializeStruct };

use crate::{
    node::{ NodeBitmapIndex, NodeId, NodeRecord },
    tag::{ TagHierarchyIndex, TagId, TagMembershipIndex, TagPathIndex, TagRecord },
};

#[derive(Clone, Debug, Default)]
pub struct Repository {
    pub nodes: HashMap<NodeId, NodeRecord>,
    pub tags: HashMap<TagId, TagRecord>,

    // rebuildable indexes
    pub node_index: NodeBitmapIndex,
    pub tag_paths: TagPathIndex,
    pub tag_hierarchy: TagHierarchyIndex,
    pub tag_membership: TagMembershipIndex,
}

#[derive(Deserialize)]
struct RepositorySerde {
    nodes: HashMap<NodeId, NodeRecord>,
    tags: HashMap<TagId, TagRecord>,
}

impl Repository {
    // ----------------------------
    // Construction / persistence
    // ----------------------------

    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_from_json(json: &str) -> Result<Self, RepoError> {
        let repo: Repository = serde_json::from_str(json).map_err(|_| RepoError::Serialization)?;
        Ok(repo)
    }

    pub fn save_to_json(&self) -> Result<String, RepoError> {
        let json = serde_json::to_string_pretty(self).map_err(|_| RepoError::Serialization)?;
        Ok(json)
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
        todo!("direct_nodes from nodes[*].tags; subtree_nodes via hierarchy closure");
    }

    // ----------------------------
    // Tag operations
    // ----------------------------

    pub fn upsert_tag(&mut self, tag: TagRecord) -> Result<TagId, RepoError> {
        let tag_id = *tag.get_id();
        self.tags.insert(tag_id, tag);

        // rebuild indexes

        Ok(tag_id)
    }

    pub fn delete_tag(&mut self, _tag: TagId) -> Result<(), RepoError> {
        todo!("tombstone tag; remove from nodes; rebuild membership");
    }

    pub fn get_tag(&self, tag: TagId) -> Option<TagRecord> {
        self.tags.get(&tag).cloned()
    }

    pub fn get_tag_by_path(&mut self, path: Vec<String>) -> Result<TagRecord, RepoError> {
        for tag in self.tags.values() {
            if tag.get_path() == &path {
                return Ok(tag.clone());
            }
        }

        Err(RepoError::NotFound)
    }

    pub fn set_tag_path(&mut self, _tag: TagId, _new_path: Vec<&str>) -> Result<(), RepoError> {
        todo!(
            "update tag.path; dedup by path; then rebuild affected indexes (likely hierarchy + subtree)"
        );
    }

    // ----------------------------
    // Node operations
    // ----------------------------

    pub fn upsert_node(&mut self, node: NodeRecord) -> Result<(), RepoError> {
        self.nodes.insert(*node.get_id(), node);

        // self.rebuild_all_indexes();

        Ok(())
    }

    pub fn delete_node(&mut self, _node: NodeId) -> Result<(), RepoError> {
        todo!("mark deleted; update membership bitmaps");
    }

    pub fn get_node(&self, node: NodeId) -> Option<&NodeRecord> {
        self.nodes.get(&node)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &NodeRecord> {
        self.nodes.values()
    }

    // ----------------------------
    // Tagging operations
    // ----------------------------

    pub fn tag_node(&mut self, node: NodeId, tag: TagId) -> Result<(), RepoError> {
        // todo!("update node.tags; update membership indexes");

        let node = self.nodes.get_mut(&node).ok_or(RepoError::NotFound)?;

        if !node.get_tags().contains(&tag) {
            node.tags.push(tag);
        }

        Ok(())
    }

    pub fn untag_node(&mut self, _node: NodeId, _tag: TagId) -> Result<(), RepoError> {
        todo!("update node.tags; update membership indexes");
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
        _bm: &'a RoaringBitmap
    ) -> impl Iterator<Item = NodeId> + 'a {
        todo!("map NodeIx -> NodeId; skip deleted nodes as desired");
        #[allow(unreachable_code)]
        std::iter::empty::<NodeId>()
    }
}

impl Serialize for Repository {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Repository", 2)?;

        state.serialize_field("nodes", &self.nodes).unwrap();
        state.serialize_field("tags", &self.tags).unwrap();
        state.end()
    }
}

impl<'de> Deserialize<'de> for Repository {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let serde_repo = RepositorySerde::deserialize(deserializer)?;

        Ok(Self {
            nodes: serde_repo.nodes,
            tags: serde_repo.tags,
            node_index: Default::default(),
            tag_paths: Default::default(),
            tag_hierarchy: Default::default(),
            tag_membership: Default::default(),
        })
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
    #[error("other: {0}")] Other(String),
}

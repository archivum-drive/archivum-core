use std::collections::HashMap;

use roaring::RoaringBitmap;
use serde::{ Deserialize, Serialize, ser::SerializeStruct };

use crate::{
    node::{ NodeId, NodeRecord },
    tag::{ TagHierarchyIndex, TagId, TagMembershipIndex, TagPathIndex, TagRecord },
};

#[derive(Deserialize)]
struct RepositorySerde {
    nodes: HashMap<NodeId, NodeRecord>,
    tags: HashMap<TagId, TagRecord>,
}

#[derive(Clone, Debug, Default)]
pub struct Repository {
    pub nodes: HashMap<NodeId, NodeRecord>,
    pub tags: HashMap<TagId, TagRecord>,

    // rebuildable indexes
    pub tag_paths: TagPathIndex,
    pub tag_hierarchy: TagHierarchyIndex,
    pub tag_membership: TagMembershipIndex,
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
        self.rebuild_tag_hierarchy_from_paths();
        self.rebuild_tag_membership_indexes();
    }

    pub fn rebuild_tag_path_index(&mut self) {
        let mut path_map: HashMap<String, TagId> = HashMap::new();

        for tag in self.iter_tags() {
            let path_str = tag.get_path().join("/");
            path_map.insert(path_str, *tag.get_id());
        }

        self.tag_paths.by_path = path_map;
    }

    pub fn rebuild_tag_hierarchy_from_paths(&mut self) {
        let mut parent_map: HashMap<TagId, Option<TagId>> = HashMap::new();
        let mut children_map: HashMap<TagId, Vec<TagId>> = HashMap::new();

        for tag in self.iter_tags() {
            let path = tag.get_path();
            let tag_id = *tag.get_id();

            let parent_id = if path.len() == 1 {
                None
            } else {
                let parent_path = &path[..path.len() - 1];

                // optimization: HashMap for path -> TagId?
                let parent_tag = self.tags.values().find(|t| t.get_path() == parent_path);
                parent_tag.map(|t| *t.get_id())
            };
            parent_map.insert(tag_id, parent_id);

            if let Some(parent_id) = parent_id {
                children_map.entry(parent_id).or_default().push(tag_id);
            }
        }
    }

    pub fn rebuild_tag_membership_indexes(&mut self) {
        let mut direct_nodes: HashMap<TagId, RoaringBitmap> = HashMap::new();
        let mut subtree_nodes: HashMap<TagId, RoaringBitmap> = HashMap::new();

        for node in self.iter_nodes() {
            for tag_id in node.get_tags() {
                direct_nodes.entry(*tag_id).or_default().insert(node.get_id().0);

                // propagate to ancestors
                let mut current_tag_id = *tag_id;
                while let Some(parent_id_opt) = self.tag_hierarchy.parent.get(&current_tag_id) {
                    if let Some(parent_id) = parent_id_opt {
                        subtree_nodes.entry(*parent_id).or_default().insert(node.get_id().0);
                        current_tag_id = *parent_id;
                    } else {
                        break;
                    }
                }
            }
        }

        self.tag_membership.direct_nodes = direct_nodes;
        self.tag_membership.subtree_nodes = subtree_nodes;
    }

    // ----------------------------
    // Tag operations
    // ----------------------------

    pub fn upsert_tag(&mut self, tag: TagRecord) -> Result<TagId, RepoError> {
        let tag_id = *tag.get_id();
        self.tags.insert(tag_id, tag);

        self.rebuild_all_indexes();

        Ok(tag_id)
    }

    pub fn delete_tag(&mut self, tag: TagId) -> Result<(), RepoError> {
        let tag = self.tags.get_mut(&tag).ok_or(RepoError::NotFound)?;
        tag.deleted = true;

        self.rebuild_all_indexes();
        Ok(())
    }

    pub fn get_tag(&self, tag: TagId) -> Option<TagRecord> {
        self.tags.get(&tag).cloned()
    }

    pub fn iter_tags(&self) -> impl Iterator<Item = &TagRecord> {
        self.tags.values().filter(|tag| !tag.deleted)
    }

    pub fn get_tag_by_path(&mut self, path: Vec<String>) -> Result<TagRecord, RepoError> {
        let path_str = path.join("/");
        if let Some(tag_id) = self.tag_paths.by_path.get(&path_str) {
            if let Some(tag) = self.tags.get(tag_id) {
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

        self.rebuild_all_indexes();

        Ok(())
    }

    pub fn delete_node(&mut self, node: NodeId) -> Result<(), RepoError> {
        let node = self.nodes.get_mut(&node).ok_or(RepoError::NotFound)?;
        node.deleted = true;

        self.rebuild_all_indexes();
        Ok(())
    }

    pub fn get_node(&self, node: NodeId) -> Option<&NodeRecord> {
        self.nodes.get(&node)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &NodeRecord> {
        self.nodes.values().filter(|node| !node.deleted)
    }

    // ----------------------------
    // Tagging operations
    // ----------------------------

    pub fn tag_node(&mut self, node: NodeId, tag: TagId) -> Result<(), RepoError> {
        let node = self.nodes.get_mut(&node).ok_or(RepoError::NotFound)?;

        if self.tags.get(&tag).is_none() {
            return Err(RepoError::NotFound);
        }

        if !node.get_tags().contains(&tag) {
            node.tags.push(tag);
        }

        self.rebuild_all_indexes();

        Ok(())
    }

    pub fn untag_node(&mut self, _node: NodeId, _tag: TagId) -> Result<(), RepoError> {
        let node = self.nodes.get_mut(&_node).ok_or(RepoError::NotFound)?;
        if self.tags.get(&_tag).is_none() {
            return Err(RepoError::NotFound);
        }

        node.tags.retain(|t| *t != _tag);

        self.rebuild_all_indexes();
        Ok(())
    }

    // ----------------------------
    // Queries
    // ----------------------------

    pub fn get_nodes_with_tag(&self, tag: TagId) -> Option<&RoaringBitmap> {
        self.tag_membership.direct_nodes.get(&tag)
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

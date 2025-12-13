use std::{collections::HashMap, fmt::Debug};

use getset::Getters;
use smallvec::SmallVec;

use crate::tag::TagId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub u128);

/// Dense bitmap index used for membership bitmaps.
pub type NodeIx = u32;

#[derive(Clone, Debug, Getters)]
#[getset(get = "pub with_prefix")]
pub struct NodeRecord {
    id: NodeId,

    deleted: bool,

    pub(crate) tags: SmallVec<[TagId; 4]>,

    date_created: String,
    date_updated: String,
}

impl NodeRecord {
    pub fn new(
        id: NodeId,
        tags: SmallVec<[TagId; 4]>,
        date_created: String,
        date_updated: String,
    ) -> Self {
        Self {
            id,
            deleted: false,
            tags,
            date_created,
            date_updated,
        }
    }
}

/// Dense node indexing for bitmap usage (rebuildable).
#[derive(Clone, Debug, Default)]
pub struct NodeBitmapIndex {
    pub node_to_ix: HashMap<NodeId, NodeIx>,
    pub ix_to_node: Vec<NodeId>,
}

use std::{ fmt::Debug };

use getset::Getters;
use smallvec::SmallVec;

use crate::{ node_type::NodeType, tag::TagId };

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct NodeId(pub u32);

impl From<u32> for NodeId {
    fn from(value: u32) -> Self {
        NodeId(value)
    }
}

#[derive(Clone, Debug, Getters, serde::Serialize, serde::Deserialize)]
#[getset(get = "pub with_prefix")]
pub struct NodeRecord {
    id: NodeId,

    pub(crate) deleted: bool,

    pub data_ref: NodeType,
    pub(crate) tags: SmallVec<[TagId; 4]>,

    date_created: String,
    date_updated: String,
}

impl NodeRecord {
    pub fn new(
        id: NodeId,
        data: NodeType,
        tags: SmallVec<[TagId; 4]>,
        date_created: String,
        date_updated: String
    ) -> Self {
        Self {
            id,
            deleted: false,
            data_ref: data,
            tags,
            date_created,
            date_updated,
        }
    }
}

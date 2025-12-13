use crate::node::NodeRecord;

#[allow(dead_code)]
pub struct File {
    node: NodeRecord,
    path: String,
    size: u64,
}

#[allow(dead_code)]
pub struct Bookmark {
    node: NodeRecord,
    url: String,
    title: Option<String>,
}

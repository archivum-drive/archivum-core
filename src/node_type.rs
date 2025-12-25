use getset::Getters;

use crate::blob::DataBlob;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    File(File),
    Bookmark(Bookmark),
}

#[derive(Clone, Debug, Getters, serde::Serialize, serde::Deserialize)]

#[getset(get = "pub with_prefix")]
pub struct File {
    filename: String,
    mime_type: Option<String>,
    data_ref: DataBlob,
}

impl File {
    pub fn new(filename: String, mime_type: Option<String>, data_ref: DataBlob) -> Self {
        Self { filename, mime_type, data_ref }
    }
}

#[derive(Clone, Debug, Getters, serde::Serialize, serde::Deserialize)]
#[getset(get = "pub with_prefix")]
pub struct Bookmark {
    data_ref: DataBlob,
    title: Option<String>,
}

impl Bookmark {
    pub fn new(data_ref: DataBlob, title: Option<String>) -> Self {
        Self { data_ref, title }
    }
}

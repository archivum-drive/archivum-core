use getset::Getters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    File(File),
    Bookmark(Bookmark),
}

#[derive(Clone, Debug, Getters, serde::Serialize, serde::Deserialize)]

#[getset(get = "pub with_prefix")]
pub struct File {
    path: String,
    size: u64,
}

impl File {
    pub fn new(path: String, size: u64) -> Self {
        Self { path, size }
    }
}

#[derive(Clone, Debug, Getters, serde::Serialize, serde::Deserialize)]
#[getset(get = "pub with_prefix")]
pub struct Bookmark {
    url: String,
    title: Option<String>,
}

impl Bookmark {
    pub fn new(url: String, title: Option<String>) -> Self {
        Self { url, title }
    }
}

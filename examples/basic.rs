use std::collections::HashMap;

use archivum_core::{
    blob::{ BlobId, BlobStore },
    node::{ NodeId, NodeRecord },
    node_type::{ Bookmark, NodeType },
    state::repository::Repository,
    tag::{ TagId, TagRecord },
};
use smallvec::smallvec;

struct InMemoryStore(HashMap<BlobId, Vec<u8>>);

impl BlobStore for InMemoryStore {
    type Error = String;
    fn upload(&mut self, id: &BlobId, data: &[u8]) -> Result<(), Self::Error> {
        self.0.insert(id.clone(), data.to_vec());
        Ok(())
    }
    fn download(&self, id: &BlobId) -> Result<Vec<u8>, Self::Error> {
        self.0
            .get(id)
            .cloned()
            .ok_or_else(|| "Blob not found".to_string())
    }
}

fn main() {
    let mut repo = Repository::new();
    let mut store = InMemoryStore(HashMap::new());

    {
        let blob = repo.upload_data(&mut store, "https://example.com".as_bytes()).unwrap();

        let data = NodeType::Bookmark(Bookmark::new(blob, Some("Example Site".to_string())));
        let node = NodeRecord::new(
            NodeId(0),
            data,
            smallvec![],
            "2025-01-01".to_string(),
            "2025-01-01".to_string()
        );

        repo.upsert_node(node).unwrap();
    }

    {
        let tag = TagRecord::new(TagId(0), vec!["photos".to_string()], None);

        repo.upsert_tag(tag).unwrap();
    }

    repo.tag_node(NodeId(0), TagId(0)).unwrap();

    for n in repo.iter_nodes() {
        println!("{:?}", n);
    }

    println!();

    // println!("{}", repo.save_to_json().unwrap());

    let node = repo.get_node(NodeId(0)).unwrap();
    let bookmark = node.get_data_ref();

    if let NodeType::Bookmark(bm) = bookmark {
        let blob = bm.get_data_ref();
        let data = blob.retrieve_data(&store).unwrap();

        let url = String::from_utf8(data).unwrap();
        println!("Bookmark URL: {}", url);
    }
}

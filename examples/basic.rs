use archivum_core::{
    node::{ NodeId, NodeRecord },
    node_type::{ Bookmark, NodeType },
    repository::Repository,
    tag::{ TagId, TagRecord },
};
use smallvec::smallvec;

fn main() {
    let mut repo = Repository::new();

    {
        let node = NodeRecord::new(
            NodeId(0),
            NodeType::Bookmark(
                Bookmark::new("https://example.com".to_string(), Some("Example Site".to_string()))
            ),
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

    println!("{}", repo.save_to_json().unwrap());
}

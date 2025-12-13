use archivum_core::{
    node::{NodeId, NodeRecord},
    repository::Repository,
    tag::{TagId, TagRecord},
};
use smallvec::smallvec;

fn main() {
    let mut repo = Repository::new();

    {
        let node = NodeRecord::new(
            NodeId(0),
            smallvec![],
            "2025-01-01".to_string(),
            "2025-01-01".to_string(),
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
}

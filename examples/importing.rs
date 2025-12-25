use archivum_core::{ state::repository::Repository };

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json: &str = &args[1];

    let repo = Repository::load_from_json(json).unwrap();

    for n in repo.iter_nodes() {
        println!("{:?}", n);
    }
}

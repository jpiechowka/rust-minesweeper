extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.to_ascii_lowercase().contains("windows") {
        embed_resource::compile("icons/icon.rc", embed_resource::NONE);
    }
}

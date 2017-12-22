extern crate solc;

fn main() {
    solc::compile(concat!(env!("CARGO_MANIFEST_DIR"), "/contracts/"));
}

use std::env;

mod analyzer;

fn main() {
    let path: String = env::args()
        .last()
        .expect("Must supply path to .jack file or directory containing .jack files");

    analyzer::read_input(&path)
}

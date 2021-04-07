extern crate jack_compiler;
use jack_compiler::analyzer;
use std::env;

fn main() {
    let path: String = env::args()
        .last()
        .expect("Must supply path to .jack file or directory containing .jack files");

    analyzer::tokenize_files(&path)
}
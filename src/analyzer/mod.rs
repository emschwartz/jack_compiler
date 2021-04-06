use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::path::Path;

mod compilation_engine;
mod tokenizer;
mod types;

use tokenizer::tokenize_lines;
use types::Token;

pub fn read_input(filename: &str) {
  let path = Path::new(filename);
  if path.is_dir() {
    let files = path
      .read_dir()
      .expect(&format!("Cannot read directory: {}", filename));
    for file in files {
      let file = file.unwrap().path();
      let output_file = path.with_extension("xml");
      let tokens = read_file(&file);
    }
  } else if path.extension().unwrap().to_str().unwrap() == "jack" {
    let output_file = path.with_extension("xml");
    let tokens: Vec<Token> = read_file(&path).collect();
    dbg!(tokens);
  } else {
    panic!("Must supply either a .jack file or a directory containing .jack files");
  }
}

fn read_file(path: &Path) -> impl Iterator<Item = Token> {
  let file = File::open(path).expect(&format!("Cannot open file: {}", path.to_str().unwrap()));
  let reader = BufReader::new(file);
  let lines = reader.lines().filter_map(|line| line.ok());
  let tokens = tokenize_lines(lines);
  tokens
}

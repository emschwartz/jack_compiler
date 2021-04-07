use std::fs::{File};
use std::io::{BufRead, BufReader, Write};
use std::iter::Iterator;
use std::path::Path;

mod compilation_engine;
mod tokenizer;
mod types;

use tokenizer::tokenize;
use types::{Token, ToXml};

pub fn tokenize_files(path: &str) {
  let path = Path::new(path);
  let files = if path.is_dir() {
    path
      .read_dir()
      .expect(&format!("Cannot read directory: {}", path.to_str().unwrap()))
      .map(|dir_entry| dir_entry.unwrap().path())
      .collect()
  } else {
    vec![path.to_owned()]
  };
  for file in files {
    if file.extension().and_then(|s| s.to_str()) == Some("jack") {
      tokenize_file(&file)
    }
  }
}

fn tokenize_file(path: &Path) {
  let file = File::open(path).expect(&format!("Cannot open file: {}", path.to_str().unwrap()));
  let output_path = &path.with_file_name(format!("{}T_out.xml", path.file_stem().and_then(|p| p.to_str()).unwrap()));
  let reader = BufReader::new(file);
  let lines = reader.lines().filter_map(|line| line.ok());
  let tokens = tokenize(lines);
  let mut output_file = File::create(output_path).expect("Unable to create file");
  write!(output_file, "{}",  &tokens.collect::<Vec<Token>>().to_xml()).expect("Error writing to file");
}

use jack_compiler::{ToXml, tokenizer::{tokenize, Token}, parser::parse};
use std::fs::{File, create_dir_all};
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::iter::Iterator;
use std::path::Path;
use clap::{Arg, App};

fn main() {
  let matches = App::new("jackc")
    .about("Jack compiler")
    .arg(Arg::with_name("input_path")
        .index(1)
        .help("Jack file or directory with jack files to compile")
        .required(true))
    .arg(Arg::with_name("tokenize")
        .short("t")
        .long("tokenize")
        .help("Output an XML file with the unparsed input tokens"))
    .arg(Arg::with_name("parse")
        .short("p")
        .long("parse")
        .help("Output an XML file with the parsed program"))
    .arg(Arg::with_name("output_dir")
        .short("o")
        .long("output_dir")
        .help("Specify the output directory for the compiled files")
        .takes_value(true))
    .get_matches();

    let path = matches.value_of("input_path").unwrap();
    let output_tokens = matches.is_present("tokenize");
    let output_parsed = matches.is_present("parse");
    let output_dir = Path::new(matches.value_of("output_dir").unwrap_or(path));
    create_dir_all(output_dir).expect("Error creating output directory");

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
      if file.extension().and_then(|s| s.to_str()) != Some("jack") {
        continue;
      }

      let file = File::open(file)
        .expect(&format!("Cannot open file: {}", path.to_str().unwrap()));
      let reader = BufReader::new(file);
      let lines = reader
        .lines()
        .map(|line| line.expect("Error reading line"));

      let mut output_tokens_file = if output_tokens {
        let output_path= &output_dir
          .with_file_name(format!("{}T.xml", path.file_stem().and_then(|p| p.to_str()).unwrap()));
        let output_file = File::create(output_path).expect("Unable to create file");
        let mut writer = BufWriter::new(output_file);
        write!(writer, "<tokens>\n").expect("Error writing to tokens file");
        Some(writer)
      } else {
        None
      };

      let tokens = tokenize(lines)
          .inspect(|token| {
            if let Some(ref mut writer) = output_tokens_file {
              write!(writer, "{}\n", token.to_xml()).expect("Error writing token to file");
            }
          });

      let parsed = parse(tokens).expect("Error compiling class");
      if let Some(ref mut writer) = output_tokens_file {
        write!(writer, "</tokens>").expect("Error writing to tokens file");
      }

      if output_parsed {
        let output_path = &output_dir
          .with_file_name(path.with_extension("xml").file_name().and_then(|p| p.to_str()).unwrap());
        let mut output_file = File::create(output_path).expect("Unable to create file");
        write!(output_file, "{}", parsed.to_xml()).expect("Error writing parsed tokens to file");
      }
    }
}
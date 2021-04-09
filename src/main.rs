use clap::{App, Arg};
use jack_compiler::{parser::parse, tokenizer::tokenize, ToXml};
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::iter::Iterator;
use std::path::Path;

fn main() {
    let matches = App::new("jackc")
        .about("Jack compiler")
        .arg(
            Arg::with_name("input_path")
                .index(1)
                .help("Jack file or directory with jack files to compile")
                .required(true),
        )
        .arg(
            Arg::with_name("tokenize")
                .short("t")
                .long("tokenize")
                .help("Output an XML file with the unparsed input tokens"),
        )
        .arg(
            Arg::with_name("parse")
                .short("p")
                .long("parse")
                .help("Output an XML file with the parsed program"),
        )
        .arg(
            Arg::with_name("output_dir")
                .short("o")
                .long("output_dir")
                .help("Specify the output directory for the compiled files")
                .takes_value(true),
        )
        .get_matches();

    let path = matches.value_of("input_path").unwrap();
    let output_tokens = matches.is_present("tokenize");
    let output_parsed = matches.is_present("parse");
    let output_dir = Path::new(matches.value_of("output_dir").unwrap_or(path));
    println!("output dir {}", output_dir.to_str().unwrap());
    create_dir_all(output_dir).expect("Error creating output directory");

    let path = Path::new(path);
    let files = if path.is_dir() {
        path.read_dir()
            .expect(&format!(
                "Cannot read directory: {}",
                path.to_str().unwrap()
            ))
            .map(|dir_entry| dir_entry.unwrap().path())
            .collect()
    } else {
        vec![path.to_owned()]
    };

    for file_path in files {
        if file_path.extension().and_then(|s| s.to_str()) != Some("jack") {
            continue;
        }

        let file =
            File::open(&file_path).expect(&format!("Cannot open file: {}", path.to_str().unwrap()));
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.expect("Error reading line"));

        let mut output_tokens_file = if output_tokens {
            let output_path = &output_dir.join(format!(
                "{}T.xml",
                file_path.file_stem().and_then(|p| p.to_str()).unwrap()
            ));
            let output_file = File::create(output_path).expect("Unable to create file");
            let mut writer = BufWriter::new(output_file);
            write!(writer, "<tokens>\n").expect("Error writing to tokens file");
            Some(writer)
        } else {
            None
        };

        let tokens = tokenize(lines).inspect(|token| {
            if let Some(ref mut writer) = output_tokens_file {
                write!(writer, "{}\n", token.to_xml()).expect("Error writing token to file");
            }
        });

        let parsed = parse(tokens).expect("Error compiling class");
        if let Some(ref mut writer) = output_tokens_file {
            write!(writer, "</tokens>").expect("Error writing to tokens file");
        }

        if output_parsed {
            let output_path = &output_dir.join(
                file_path
                    .with_extension("xml")
                    .file_name()
                    .and_then(|p| p.to_str())
                    .unwrap(),
            );
            let mut output_file = File::create(output_path).expect("Unable to create file");
            let output_string = parsed.to_xml();
            // Remove empty lines
            // (this is less efficient but simpler than ensuring we exactly
            // match the spacing expected by the nand2tetris compare file)
            let output_string = output_string
                .split("\n")
                .filter(|line| !line.chars().all(|c| c.is_whitespace()))
                .collect::<Vec<&str>>()
                .join("\n");
            write!(output_file, "{}", output_string).expect("Error writing parsed tokens to file");
        }
    }
}

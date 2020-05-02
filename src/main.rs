extern crate gamemaps_parser;

use std::env::args;
use std::fs::File;
use std::io::BufReader;
use gamemaps_parser::header;
use gamemaps_parser::levels;

fn main() {
    let mut args = args();
    args.next(); // The executable is the first argument.
    let header_file_path = args.next().expect(&format!("No header file path specified."));
    let level_file_path = args.next().expect(&format!("No level data file path specified."));

    let header_file = File::open(&header_file_path).expect(&format!("Header file not found: {}", header_file_path));

    let header_reader: Box<_> = BufReader::new(header_file).into();
    let header_data = header::parse(header_reader).expect(&format!("Error while parsing the header file."));
    println!("Header file parsed successfully.");

    let mut levels_parsed_successfully = 0;
    let total_level_count = header_data.level_offsets.len();
    for level_offset in header_data.level_offsets.iter() {
        let level_file = File::open(&level_file_path).expect(&format!("Level file not found: {}", level_file_path));
        let level_reader: Box<_> = BufReader::new(level_file).into();
        match levels::parse(level_reader, *level_offset) {
            Ok(_) => {
                levels_parsed_successfully += 1;
                println!("Successfully parsed level {}/{}.", levels_parsed_successfully, total_level_count);
            },
            Err(e) => {
                println!(
                    "Parsing level data failed for level {}. Reason: {}.",
                    levels_parsed_successfully + 1,
                    e
                );
                return;
            }
        }
    }
    println!(
        "Parsed level data successfully for {} levels.",
        levels_parsed_successfully
    );
}
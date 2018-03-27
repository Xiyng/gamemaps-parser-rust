extern crate gamemaps_parser;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use gamemaps_parser::header;

fn main() {
    let mut args = args();
    args.next(); // The executable is the first argument.

    let header_file_path = match args.next() {
        Some(path) => path,
        None       => return println!("No header file path specified.")
    };

    let level_file_path = match args.next() {
        Some(path) => path,
        None       => return println!("No level data file path specified.")
    };

    let mut header_file = match File::open(&header_file_path) {
        Ok(file) => file,
        _        => return println!("Header file not found: {}", header_file_path)
    };
    let mut header_data = Vec::new();
    if header_file.read_to_end(&mut header_data).is_err() {
        return println!("Error reading header file: {}", header_file_path)
    }

    let mut level_file = match File::open(&level_file_path) {
        Ok(file) => file,
        _        => return println!("Level file not found: {}", level_file_path)
    };
    let mut level_data = Vec::new();
    if level_file.read_to_end(&mut level_data).is_err() {
        return println!("Error reading level file: {}", level_file_path)
    }

    header::parse(&header_data); // to see whether it runs without errors

    println!("Header and level file read succesfully!");
}
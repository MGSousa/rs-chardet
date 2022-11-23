extern crate chardet;

use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: detect FILE");
        std::process::exit(1);
    }

    let mut reader: Vec<u8> = Vec::new();
    let mut file = OpenOptions::new().read(true).open(&args[1])
        .expect("Could not open file");

    file.read_to_end(&mut reader)
        .expect("Could not read file");

    // Detect charset of file
    let result = chardet::detect(&reader);
    if result.0 != "" {
        println!("{:?}", result);
    }
}

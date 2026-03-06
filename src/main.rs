use std::env;
use std::fs;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let fp = fs::read_to_string(file_path).expect("Failed to read file");

    let lines: Vec<&str> = fp.split('\n').collect();

    for line in lines {
        println!("{line}");
    }
    // TODO: generate config from lines

    // TODO: determine color values at each pixel

    // TODO: write header to output file

    // TODO: write pixels to output file
}

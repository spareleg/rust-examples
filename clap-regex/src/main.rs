use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;
use regex::Regex;

/// Lite version of grep
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg()]
    pattern: String,
    /// File to search
    #[arg()]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();
    let re = Regex::new(&args.pattern).unwrap();
    if let Some(input) = args.input {
        let f = File::open(input).unwrap();
        print_matched_lines(BufReader::new(f), re);
    } else {
        print_matched_lines(io::stdin().lock(), re);
    }
}

fn print_matched_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line in reader.lines() {
        if let Ok(line) = line
            && re.is_match(&line)
        {
            println!("{line}")
        }
    }
}

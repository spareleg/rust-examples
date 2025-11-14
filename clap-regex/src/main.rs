use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    process,
};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    pattern: String,
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let Ok(re) = Regex::new(&args.pattern) else {
        eprintln!("Invalid pattern 🤨 {}", &args.pattern);
        process::exit(1);
    };

    // Read from the file if it's provided and accessible, otherwise read from standard input
    if let Some(filename) = args.filename
        && let Ok(f) = File::open(filename)
    {
        print_matched_lines(BufReader::new(f), re);
    } else {
        print_matched_lines(io::stdin().lock(), re);
    }
}

fn print_matched_lines(input: impl BufRead, re: Regex) {
    for line in input.lines() {
        if let Ok(line) = line
            && re.is_match(&line)
        {
            println!("{line}")
        }
    }
}

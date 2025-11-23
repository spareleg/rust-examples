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

    let res = if let Some(filename) = args.filename {
        // Try reading from the provided file
        File::open(filename).and_then(|f| print_matched_lines(BufReader::new(f), re))
    } else {
        // Filename is not provided so let's read from standard input
        print_matched_lines(io::stdin().lock(), re)
    };

    if let Err(err) = res {
        eprintln!("{err}");
        process::exit(1)
    }
}

fn print_matched_lines(input: impl BufRead, re: Regex) -> io::Result<()> {
    for line in input.lines() {
        let line = line?;
        if re.is_match(&line) {
            println!("{line}")
        }
    }
    Ok(())
}

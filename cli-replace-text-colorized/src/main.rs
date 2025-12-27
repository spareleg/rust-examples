use std::{error::Error, fs, process};

use clap::Parser;
use nu_ansi_term::Color::{Red, Yellow};
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    let re = Regex::new(&args.target)
        .unwrap_or_else(|e| exit(e, "failed to parse regular expression", &args.target));

    let text = fs::read_to_string(&args.filename)
        .unwrap_or_else(|e| exit(e, "failed to read from file", &args.filename));

    let text = re.replace_all(&text, &args.replacement);

    if let Err(e) = fs::write(&args.output, text.as_bytes()) {
        exit(e, "failed to write to file", &args.output)
    }
}

fn exit(err: impl Error, msg: &str, details: &str) -> ! {
    eprintln!(
        "{} {msg} {}\n{err}",
        Red.bold().paint("Error:"),
        Yellow.underline().paint(details),
    );
    process::exit(1)
}

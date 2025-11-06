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
    let re = Regex::new(&args.target).unwrap_or_else(|e| {
        let msg = format!(
            "failed to parse regular expression {}",
            Yellow.underline().paint(&args.target)
        );
        exit(&msg, e)
    });

    let text = fs::read_to_string(&args.filename).unwrap_or_else(|e| {
        let msg = format!(
            "failed to read from file {}",
            Yellow.underline().paint(&args.filename)
        );
        exit(&msg, e)
    });

    let text = re.replace_all(&text, &args.replacement).to_string();

    if let Err(e) = fs::write(&args.output, text) {
        let msg = format!(
            "failed to write to file {}",
            Yellow.underline().paint(&args.output)
        );
        exit(&msg, e)
    }
}

fn exit(msg: &str, err: impl Error) -> ! {
    eprintln!("{} {msg}\n{err}", Red.bold().paint("Error:"));
    process::exit(1)
}

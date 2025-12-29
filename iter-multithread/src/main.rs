use std::{
    collections::{BTreeSet, HashMap},
    env,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
    sync::mpsc,
    thread,
};

pub trait Spawn: Iterator {
    fn spawn(self) -> mpsc::IntoIter<Self::Item>;
}

impl<I> Spawn for I
where
    I: Iterator + Send + 'static,
    I::Item: Send,
{
    fn spawn(self) -> mpsc::IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(512);
        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });
        receiver.into_iter()
    }
}

/// cargo run Cargo.toml src/main.rs
fn main() {
    let filenames: Vec<_> = env::args().skip(1).map(PathBuf::from).collect();

    // States that are moved into closures and thereby into child threads:
    let mut ch_counts: HashMap<char, u32> = HashMap::new();
    let mut max_count = 0;
    let mut top_chars = BTreeSet::new();

    filenames
        .into_iter()
        .map(|filename| BufReader::new(File::open(filename).unwrap())) // unwrap for example simplicity, don't do this at home
        .flat_map(|buf| buf.lines().map(|l| l.unwrap())) // unwrap for example simplicity, don't do this at home
        .spawn() // reads files in a separate thread and sends them by lines
        .flat_map(|mut line| iter::from_fn(move || line.pop()))
        .filter(|ch| !ch.is_whitespace())
        .map(move |ch| {
            let count = ch_counts
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_default();
            (ch, *count)
        })
        .spawn() // counts characters in a separate thread and sends the top char with count
        // The end of the pipeline is executed on the main thread:
        .for_each(move |(ch, count)| {
            if count > max_count {
                max_count = count;
                top_chars.clear(); // Clear chars with lower count than new max
            }
            if count == max_count {
                // Adds either the char in a tie to the existing or a new top char into just emptied set
                top_chars.insert(ch);
                print!("\x1b[1A\x1b[2K\r"); // Move up one line and clear it (Linux only)
                println!("{max_count}: {}", top_chars.iter().collect::<String>());
            }
        });
}

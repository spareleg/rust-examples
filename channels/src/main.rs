use std::{
    collections::{BTreeSet, HashMap},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
};

fn main() {
    let files = env::args().skip(1).map(PathBuf::from).collect();

    let (lines, h1) = start_file_reading_thread(files);
    let (char_counts, h2) = start_char_counting_thread(lines);
    print_top_char(char_counts);

    // These unwraps propagate possible panic in the spawned threads into this thread
    let r1 = h1.join().unwrap();
    h2.join().unwrap();

    // Result propagated from the spawned thread closure
    if let Err(err) = r1 {
        eprintln!("{err}");
    }
}

fn start_file_reading_thread(
    files: Vec<PathBuf>,
) -> (Receiver<String>, JoinHandle<io::Result<()>>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for file_path in files {
            let reader = BufReader::new(File::open(file_path)?);
            for line in reader.lines() {
                if tx.send(line?).is_err() {
                    // A send call fails if the Receiver has been dropped;
                    // Dropping your end of a channel is the normal way of closing the connection when you’re done with it;
                    return Ok(());
                }
            }
        }
        Ok(())
    });

    (rx, handle)
}

fn start_char_counting_thread(
    lines: Receiver<String>,
) -> (Receiver<(char, usize)>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut ch_counts = HashMap::new();
        for line in lines {
            for ch in line.chars() {
                if ch.is_whitespace() {
                    continue;
                }
                let count = ch_counts
                    .entry(ch)
                    .and_modify(|count| *count += 1)
                    .or_default();
                if tx.send((ch, *count)).is_err() {
                    return;
                }
            }
        }
    });

    (rx, handle)
}

fn print_top_char(char_counts: Receiver<(char, usize)>) {
    let mut max_count = 0;
    let mut top_chars = BTreeSet::new();

    for (ch, count) in char_counts {
        if count > max_count {
            max_count = count;
            top_chars.clear(); // Clear chars with lower count than new max
        }
        if count == max_count {
            // Adds either the char in a tie to the existing or a new top char into just emptied set
            top_chars.insert(ch);
            print!("\x1b[1A\x1b[2K\r"); // Move up one line and clear it
            println!("{max_count}: {}", top_chars.iter().collect::<String>());
        }
    }
}

use std::{env, io, str::FromStr};

fn main() {
    let (w, h) = read_resolution();
    let d = greatest_common_divisor(w, h);
    println!("The aspect ratio of {w}x{h} is {}:{}", w / d, h / d);
}

fn read_resolution() -> (u32, u32) {
    // First try getting 2 valid integers from args
    let mut from_args = env::args().skip(1).filter_map(|a| a.parse().ok());
    if let (Some(w), Some(h)) = (from_args.next(), from_args.next()) {
        return (w, h);
    }
    // No 2 valid integers from args, ask for them through stdin
    let mut buffer = String::new();
    let is_positive = |&input: &u32| input != 0;
    let w = read_stdin("Width", &mut buffer, is_positive);
    let h = read_stdin("Height", &mut buffer, is_positive);
    (w, h)
}

fn read_stdin<F, R>(prompt: &str, buffer: &mut String, mut is_valid: F) -> R
where
    R: FromStr,
    F: FnMut(&R) -> bool,
{
    println!("Enter {prompt}");
    loop {
        buffer.clear();
        if io::stdin().read_line(buffer).is_ok()
            && let Ok(input) = buffer.trim().parse()
            && is_valid(&input)
        {
            return input;
        } else {
            eprintln!("Invalid input")
        }
    }
}

fn greatest_common_divisor(mut n: u32, mut m: u32) -> u32 {
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m %= n;
    }
    n
}

#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(1920, 1080), 120);
    assert_eq!(greatest_common_divisor(832, 1216), 64);
}

use std::{env, io};

fn main() {
    let (x, y) = read_resolution();
    let d = greatest_common_divisor(x, y);
    println!("The aspect ratio of {}x{} is: {}:{}", x, y, x / d, y / d);
}

fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m %= n;
    }
    n
}

// TODO add != 0 check
fn read_resolution<R: std::str::FromStr>() -> (R, R) {
    // First try getting 2 valid integers from args
    let mut from_args = env::args().skip(1).filter_map(|a| a.parse().ok());
    if let (Some(x), Some(y)) = (from_args.next(), from_args.next()) {
        return (x, y);
    }
    // No 2 valid integers from args, ask for them through stdin
    let mut buffer = String::new();
    let stdin = io::stdin();
    let x = read_user_input("Width", &mut buffer, &stdin);
    let y = read_user_input("Height", &mut buffer, &stdin);
    (x, y)
}

fn read_user_input<R: std::str::FromStr>(
    prompt: &str,
    buffer: &mut String,
    stdin: &io::Stdin,
) -> R {
    println!("Enter {}", prompt);
    loop {
        buffer.clear();
        if stdin.read_line(buffer).is_ok()
            && let Ok(n) = buffer.trim().parse()
        {
            return n;
        } else {
            eprintln!("a valid integer is expected")
        }
    }
}

#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(14, 15), 1);
    assert_eq!(greatest_common_divisor(1920, 1080), 120);
    assert_eq!(greatest_common_divisor(832, 1216), 64);
}

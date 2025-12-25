use std::borrow::Cow;

fn main() {
    let fizzes = (1..).map(|i| i % 3 == 0);
    let buzzes = (1..).map(|i| i % 5 == 0);
    let fizz_buzz = fizzes
        .zip(buzzes)
        .enumerate()
        .map(|(i, fizz_buzz)| -> Cow<'static, str> {
            match fizz_buzz {
                (false, false) => (i + 1).to_string().into(),
                (true, false) => "fizz".into(),
                (false, true) => "buzz".into(),
                (true, true) => "fizzbuzz".into(),
            }
        });

    for fb in fizz_buzz.take(60) {
        println!("{fb}");
    }
}

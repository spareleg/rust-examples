use num::Complex;
use std::str::FromStr;

pub fn pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    let sep_i = s.find(separator)?;
    match (s[..sep_i].parse(), s[sep_i + 1..].parse()) {
        (Ok(left), Ok(right)) => Some((left, right)),
        _ => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(pair::<i32>("", ','), None);
    assert_eq!(pair::<i32>("10,", ','), None);
    assert_eq!(pair::<i32>(",10", ','), None);
    assert_eq!(pair("10,20", ','), Some((10, 20)));
    assert_eq!(pair::<i32>("10,20xy", ','), None);
    assert_eq!(pair::<f64>("0.5x", 'x'), None);
    assert_eq!(pair("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

pub fn complex<T: FromStr>(s: &str) -> Option<Complex<T>> {
    pair(s, ',').map(|(re, im)| Complex { re, im })
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(complex::<f64>(",-0.0625"), None);
}

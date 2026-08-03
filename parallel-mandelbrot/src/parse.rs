use std::str::FromStr;

use num::Complex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pair<T>(pub T, pub T);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PairParseError<E> {
    MissingDelimiter,
    Inner(E),
}

impl<E> From<E> for PairParseError<E> {
    fn from(e: E) -> Self {
        Self::Inner(e)
    }
}

impl<T: FromStr> FromStr for Pair<T> {
    type Err = PairParseError<T::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(',')
            .or_else(|| s.split_once('x'))
            .ok_or(PairParseError::MissingDelimiter)?;
        Ok(Pair(left.parse()?, right.parse()?))
    }
}

#[test]
fn test_parse_arg_pair() {
    assert_eq!("42,69".parse(), Ok(Pair(42, 69)));
    assert_eq!("4242x69".parse(), Ok(Pair(4242, 69)));
    assert_eq!(
        "12345".parse::<Pair<u16>>(),
        Err(PairParseError::MissingDelimiter)
    );
}

impl<T> From<Pair<T>> for Complex<T> {
    fn from(pair: Pair<T>) -> Self {
        Complex {
            re: pair.0,
            im: pair.1,
        }
    }
}

#[test]
fn test_parse_complex() {
    let pair: Pair<_> = "42.69,-69.4242".parse().unwrap();
    let c: Complex<_> = pair.into();
    assert_eq!(
        c,
        Complex {
            re: 42.69,
            im: -69.4242
        }
    );
}

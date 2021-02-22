use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Read},
};

type Range = (usize, usize);

fn valid(password: usize) -> bool {
    // convert number as string, it's easier to process this way
    let digits = password.to_string();

    // two adjacent digits are the same
    let rule1 = digits.as_bytes().windows(2).any(|w| w[0] == w[1]);

    // going from left to right, the digits never decrease
    let rule2 = digits.as_bytes().windows(2).all(|w| w[0] <= w[1]);

    rule1 && rule2
}

fn valid2(password: usize) -> bool {
    // convert number as string, it's easier to process this way
    let digits = password.to_string();

    // two adjacent digits are the same, but not not part of a larger group of matching digits
    let mut digits_freq = HashMap::new();

    for w in digits.as_bytes().windows(2) {
        if w[0] == w[1] {
            *digits_freq.entry(w[0]).or_insert(0) += 1;
        }
    }

    let rule1 = digits_freq.values().any(|&x| x == 1);

    // going from left to right, the digits never decrease
    let rule2 = digits.as_bytes().windows(2).all(|w| w[0] <= w[1]);

    rule1 && rule2
}

pub fn part1(input: Range) -> usize {
    let (start, end) = input;
    (start..=end).filter(|&x| valid(x)).count()
}

pub fn part2(input: Range) -> usize {
    let (start, end) = input;
    (start..=end).filter(|&x| valid2(x)).count()
}

pub fn get_input(f: impl Read) -> Result<Range, Box<dyn Error>> {
    // read data from input.txt
    let reader = BufReader::new(f);

    // read and parse input
    let tokens = reader
        .split(b'-')
        .map(|cursor| -> Result<usize, Box<dyn Error>> {
            // read token from cursor
            let token = cursor?;

            // decode token into utf-8 string
            let s = std::str::from_utf8(&token)?.trim();

            // parse token into unsigned integer
            let value = s.parse::<usize>()?;

            Ok(value)
        })
        .take(2)
        .collect::<Result<Vec<_>, _>>()?;

    // return result as tuple
    match tokens[..] {
        [a, b] => Ok((a, b)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() {
        assert_eq!(
            (382_345, 843_167),
            get_input(Cursor::new("382345-843167")).unwrap()
        );
        assert_eq!(true, valid(122_345));
        assert_eq!(true, valid(111_111));
        assert_eq!(false, valid(223_450));
        assert_eq!(false, valid(123_789));
    }

    #[test]
    fn test_part2() {
        assert_eq!(true, valid2(122_345));
        assert_eq!(false, valid2(223_450));
        assert_eq!(false, valid2(123_789));
        assert_eq!(true, valid2(112_233));
        assert_eq!(false, valid2(123_444));
        assert_eq!(true, valid2(111_122));
    }
}

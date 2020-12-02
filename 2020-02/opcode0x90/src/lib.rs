#[macro_use]
extern crate lazy_static;

use {
    regex::Regex,
    std::{
        error::Error,
        io::{BufRead, BufReader, Read},
        num::ParseIntError,
        str::FromStr,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct Password {
    policy_min: usize,
    policy_max: usize,
    policy_char: char,
    password: String,
}

pub fn part1(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|line| {
            let count = line
                .password
                .chars()
                .filter(|&c| c == line.policy_char)
                .count();

            count >= line.policy_min && count <= line.policy_max
        })
        .count()
}

pub fn part2(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|line| {
            let a = line.policy_char == line.password.chars().nth(line.policy_min - 1).unwrap();
            let b = line.policy_char == line.password.chars().nth(line.policy_max - 1).unwrap();

            a ^ b
        })
        .count()
}

pub fn get_input(f: impl Read) -> Result<Vec<Password>, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut buffer = vec![];

    // read the input line by line
    for line in reader.lines() {
        // attempt to parse each line
        let value = line?.parse::<Password>()?;
        buffer.push(value);
    }

    Ok(buffer)
}

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"(\d+?)-(\d+?) (\w): (\w+)$").unwrap();
}
impl FromStr for Password {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE_PASSWORD.captures(s).expect("invalid input");

        Ok(Password {
            policy_min: cap.get(1).expect("invalid input").as_str().parse()?,
            policy_max: cap.get(2).expect("invalid input").as_str().parse()?,
            policy_char: cap
                .get(3)
                .expect("invalid input")
                .as_str()
                .chars()
                .next()
                .unwrap(),
            password: cap.get(4).expect("invalid input").as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static DATA: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn test_input() {
        let input = get_input(Cursor::new(DATA)).unwrap();

        assert_eq!(
            input,
            vec![
                Password {
                    policy_min: 1,
                    policy_max: 3,
                    policy_char: 'a',
                    password: "abcde".to_owned()
                },
                Password {
                    policy_min: 1,
                    policy_max: 3,
                    policy_char: 'b',
                    password: "cdefg".to_owned()
                },
                Password {
                    policy_min: 2,
                    policy_max: 9,
                    policy_char: 'c',
                    password: "ccccccccc".to_owned()
                },
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part2(&input), 1);
    }
}

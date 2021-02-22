use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

pub fn part1(input: &[String]) -> u32 {
    fn frequency_map(s: &str) -> HashMap<char, u32> {
        // compute frequency count for each characters in given string
        let mut dict = HashMap::new();

        for c in s.chars() {
            // insert char into dict if not exist, otherwise increment count
            let i = dict.entry(c).or_insert(0);
            *i += 1
        }
        dict
    }

    let mut two = 0;
    let mut three = 0;

    for line in input.into_iter() {
        // compute char frequency map
        let fm = frequency_map(line);

        // check if frequency map contains exactly two/three chars
        let mut x = 0;
        let mut y = 0;

        for v in fm.values() {
            match *v {
                2 => x = 1,
                3 => y = 1,
                _ => (),
            }
            // early termination if both condition is already met
            if x > 0 && y > 0 {
                break;
            }
        }

        two += x;
        three += y;
    }

    two * three
}

pub fn part2(input: &[String]) -> String {
    input
        .into_iter()
        .tuple_combinations()
        .find_map(|(a, b)| {
            // find both common and distinct chars
            let mut same = String::with_capacity(a.len());
            let mut diff = 0;

            for (a, b) in a.chars().zip(b.chars()) {
                if a == b {
                    same.push(a);
                } else {
                    diff += 1;

                    // early skipping if diff > 1
                    if diff > 1 {
                        break;
                    }
                }
            }

            match diff {
                1 => Some(same),
                _ => None,
            }
        })
        .expect("there is no candidate with distance of one!")
}

pub fn get_input(f: impl Read) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // read data from input.txt
    let input = BufReader::new(f).lines().collect::<Result<Vec<_>, _>>()?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const DATA: &str = r#"
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
        "#;
        let input = get_input(DATA.as_bytes()).unwrap();
        assert_eq!(12, part1(&input))
    }

    #[test]
    fn test_part2() {
        const DATA: &str = r#"
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
        "#;
        let input = get_input(DATA.as_bytes()).unwrap();
        assert_eq!("fgij", part2(&input))
    }
}

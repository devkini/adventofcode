use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Read},
};

pub fn part1(input: &[String]) -> usize {
    let mut count = 0;
    let mut buf = vec![];

    for line in input.iter().chain(&[String::default()]) {
        if line.is_empty() {
            buf.sort_unstable();
            buf.dedup();
            count += buf.len();
            buf.clear();
        } else {
            let mut chars = line.chars().collect::<Vec<char>>();
            buf.append(&mut chars);
        }
    }

    count
}

pub fn part2(input: &[String]) -> usize {
    let mut count = 0;

    let mut group_size = 0;
    let mut fq: HashMap<char, usize> = HashMap::new();

    for line in input.iter().chain(&[String::default()]) {
        if line.is_empty() {
            for (_, &freq) in fq.iter() {
                if freq == group_size {
                    count += 1;
                }
            }
            fq.clear();
            group_size = 0;
        } else {
            let mut chars = line.chars().collect::<Vec<_>>();

            chars.sort_unstable();
            chars.dedup();

            for c in chars.into_iter() {
                *fq.entry(c).or_insert(0) += 1;
            }
            group_size += 1;
        }
    }

    count
}

pub fn get_input(f: impl Read) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let buffer = reader.lines().collect::<Result<Vec<String>, _>>()?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static DATA: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn test_part1() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part2(&input), 6);
    }
}

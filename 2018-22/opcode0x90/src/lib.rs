use std::error::Error;
use std::io::{BufReader, Read};

pub fn part1(input: &str) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    0
}

pub fn get_input(f: impl Read) -> Result<String, Box<dyn Error>> {
    // read data from input.txt
    let mut buf = String::new();
    BufReader::new(f).read_to_string(&mut buf)?;

    // remove whitespaces
    let input = String::from(buf.trim());
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {}

    #[test]
    fn test_part2() {}
}

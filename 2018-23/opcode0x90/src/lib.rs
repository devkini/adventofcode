use std::error::Error;
use std::io::{BufRead, BufReader, Read};

use regex::Regex;

#[derive(Debug)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug)]
pub struct Nanobot {
    coord: Point,
    r: usize,
}

/// compute Manhatten distance for 2 Points
fn manhatten(a: &Point, b: &Point) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as usize
}

pub fn part1(input: &[Nanobot]) -> usize {
    // locate Nanobot with strongest signal radius
    let a = input
        .iter()
        .max_by_key(|x| x.r)
        .expect("unable to find the strongest bot!");

    // find number of nearest neighbour around this bot
    input
        .into_iter()
        .filter(|b| manhatten(&a.coord, &b.coord) <= a.r)
        .count()
}

pub fn part2(_input: &[Nanobot]) -> usize {
    42
}

pub fn get_input(f: impl Read) -> Result<Vec<Nanobot>, Box<dyn Error>> {
    // parse data from input.txt
    let re = Regex::new(r"^pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)$")?;

    BufReader::new(f)
        .lines()
        .map(|line| -> Result<Option<Nanobot>, Box<dyn Error>> {
            // attempt to parse the current line with given regex
            if let Some(parsed) = re.captures(line?.as_str()) {
                let try_extract = |n| -> Result<_, Box<dyn Error>> {
                    Ok(parsed.get(n).ok_or_else(|| "malformed input")?.as_str())
                };

                // extract Nanobot struct and parse them
                Ok(Some(Nanobot {
                    coord: Point {
                        x: try_extract(1)?.parse()?,
                        y: try_extract(2)?.parse()?,
                        z: try_extract(3)?.parse()?,
                    },
                    r: try_extract(4)?.parse()?,
                }))
            } else {
                // empty line or malformed input, ignore
                Ok(None)
            }
        })
        // drop empty Option<Nanobot>
        .filter_map(|x| {
            // transpose the Result<Option<T>> into Option<Result<T>>
            match x {
                Ok(Some(x)) => Some(Ok(x)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const DATA: &str = r#"
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
"#;

        let input = get_input(DATA.as_bytes()).expect("unable to parse input");
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn test_part2() {
        const DATA: &str = r#"
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
"#;

        let input = get_input(DATA.as_bytes()).expect("unable to parse input");
        assert_eq!(36, part2(&input));
    }
}

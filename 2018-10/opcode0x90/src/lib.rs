use std::error::Error;
use std::io::{BufRead, BufReader, Read};

use ndarray::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}
impl Point {
    /// advance the point by one time step
    fn advance(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

pub fn part1(input: &[Point]) -> usize {
    let mut points = Vec::from(input);
    let mut seconds = 0;

    // run the simulation
    for _ in 0..10000 {
        for p in points.iter_mut() {
            // advance the points
            p.advance();
        }
        // advance the time
        seconds += 1;
    }
    seconds
}

pub fn part2(_input: &[Point]) -> usize {
    0
}

pub fn get_input(f: impl Read) -> Result<Vec<Point>, Box<dyn Error>> {
    // read data from input.txt
    let input = BufReader::new(f).lines();

    // parse input into events
    let re = Regex::new(r"position=<\s*(-?\d+?),\s*(-?\d+?)> velocity=<\s*(-?\d+?),\s*(-?\d+?)>")?;
    let mut points = vec![];

    for line in input {
        if let Some(parsed) = re.captures(line?.as_str()) {
            let try_parse = |n| -> Result<_, Box<dyn Error>> {
                Ok(parsed
                    .get(n)
                    .ok_or_else(|| "malformed input")?
                    .as_str()
                    .parse()?)
            };
            points.push(Point {
                x: try_parse(1)?,
                y: try_parse(2)?,
                dx: try_parse(3)?,
                dy: try_parse(4)?,
            })
        }
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
        "#;

    #[test]
    fn test_part1() {
        let input = get_input(DATA.as_bytes()).unwrap();
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_part2() {}
}

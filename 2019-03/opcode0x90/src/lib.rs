use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{BufRead, BufReader, Read},
    iter::FromIterator,
    str::FromStr,
};

type Coord = (isize, isize);
type Trace = HashMap<Coord, usize>;

#[derive(Debug)]
pub struct Turn {
    direction: char,
    distance: usize,
}

impl FromStr for Turn {
    type Err = Box<dyn Error>;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(turn: &str) -> Result<Self, Self::Err> {
        let direction = turn
            .chars()
            .next()
            .ok_or("unable to extract the direction")?;
        let distance = turn
            .get(1..)
            .ok_or("unable to extract the distance")?
            .parse::<usize>()?;

        Ok(Turn {
            direction,
            distance,
        })
    }
}

fn trace(turns: &[Turn]) -> Trace {
    // begin tracing the wire
    let (mut x, mut y) = (0, 0);
    let mut step = 0;
    let mut trace = Trace::new();

    for turn in turns {
        // march towards destination unknown
        for _ in 0..turn.distance {
            // hold on... which direction are we going again?
            match turn.direction {
                'U' => y += 1,       // going up
                'D' => y -= 1,       // going down
                'L' => x -= 1,       // going left
                'R' => x += 1,       // going right
                _ => unreachable!(), // invalid direction
            }
            // mark the path
            // println!("coord: ({}, {})", x, y);
            let coord = (x, y);
            step += 1;
            trace.insert(coord, step);
        }
    }

    trace
}

pub fn part1(input: &[Vec<Turn>]) -> isize {
    // println!("{:?}", input);

    // input should only be 2 wires
    assert_eq!(2, input.len());

    // begin tracing both wires
    let wire1 = trace(&input[0]);
    let wire2 = trace(&input[1]);

    // extract the path
    let path1: HashSet<Coord> = HashSet::from_iter(wire1.keys().cloned());
    let path2 = HashSet::from_iter(wire2.keys().cloned());

    // find wires crossing that is closes to origin
    path1
        .intersection(&path2)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

pub fn part2(input: &[Vec<Turn>]) -> usize {
    // input should only be 2 wires
    assert_eq!(2, input.len());

    // begin tracing both wires
    let wire1 = trace(&input[0]);
    let wire2 = trace(&input[1]);

    // extract the path
    let path1: HashSet<Coord> = HashSet::from_iter(wire1.keys().cloned());
    let path2 = HashSet::from_iter(wire2.keys().cloned());

    // find wires crossing
    let crossing = path1.intersection(&path2);

    // find wires crossing with smallest sum of steps
    crossing
        .map(|coord| {
            let step1 = wire1[coord];
            let step2 = wire2[coord];

            step1 + step2
        })
        .min()
        .unwrap()
}

pub fn get_input(f: impl Read) -> Result<Vec<Vec<Turn>>, Box<dyn Error>> {
    // read data from input.txt
    let reader = BufReader::new(f);

    // read and parse input line by line
    reader
        .lines()
        .map(|line| {
            let line = line?;

            // tokenize the line delimited by comma
            let token = line.trim().split(',');

            // extract token into Turn struct
            token
                .map(|token| Turn::from_str(token))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() {
        assert_eq!(
            6,
            part1(
                &get_input(Cursor::new(
                    "R8,U5,L5,D3
                     U7,R6,D4,L4"
                ))
                .unwrap()
            )
        );
        assert_eq!(
            159,
            part1(
                &get_input(Cursor::new(
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72
                     U62,R66,U55,R34,D71,R55,D58,R83"
                ))
                .unwrap()
            )
        );
        assert_eq!(
            135,
            part1(
                &get_input(Cursor::new(
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                ))
                .unwrap()
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            30,
            part2(
                &get_input(Cursor::new(
                    "R8,U5,L5,D3
                     U7,R6,D4,L4"
                ))
                .unwrap()
            )
        );
        assert_eq!(
            610,
            part2(
                &get_input(Cursor::new(
                    "R75,D30,R83,U83,L12,D49,R71,U7,L72
                     U62,R66,U55,R34,D71,R55,D58,R83"
                ))
                .unwrap()
            )
        );
        assert_eq!(
            410,
            part2(
                &get_input(Cursor::new(
                    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                ))
                .unwrap()
            )
        );
    }
}

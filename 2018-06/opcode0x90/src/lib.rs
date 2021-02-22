use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point {
    x: usize,
    y: usize,
}

type Centroid = usize;
type Grid = HashMap<Point, Centroid>;

/// compute Manhatten distance for 2 Points
fn manhatten(a: &Point, b: &Point) -> usize {
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize
}

/// visualize
#[allow(dead_code)]
fn visualize(grid: &Grid, centroids: &[Point], n: usize) {
    let mut matrix = vec![vec!['.'; n]; n];

    // map the grid onto a 2D matrix
    for (&coord, &centroid) in grid.into_iter() {
        if centroid > 0 && coord.x < n && coord.y < n {
            let mut c = (b'a' + (centroid - 1) as u8) as char;

            if centroids.contains(&coord) {
                c = c.to_ascii_uppercase();
            }

            matrix[coord.y as usize][coord.x as usize] = c;
        }
    }

    // visualize the 2D matrix
    for vector in matrix.into_iter() {
        println!("{}", vector.into_iter().join(""));
    }
}

pub fn part1(input: &[Point]) -> usize {
    let mut area: HashMap<Centroid, usize> = HashMap::new();
    let mut unbounded: HashSet<Centroid> = HashSet::new();

    // determine the min/max of X and Y
    let mut min_x = std::usize::MAX;
    let mut min_y = std::usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for point in input.into_iter() {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    // scan grid pattern from min X+Y to max X+Y
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let point = Point { x, y };

            // compute Manhatten distance against all neighbours
            let mut neighbours: BTreeMap<usize, Vec<Centroid>> = BTreeMap::new();

            for (centroid, neighbour) in input.into_iter().enumerate() {
                let centroid = centroid + 1;
                let distance = manhatten(&point, neighbour);

                let nn = neighbours.entry(distance).or_insert_with(Vec::new);
                nn.push(centroid);
            }

            // find the nearest neighbour
            let (_, nn) = neighbours
                .into_iter()
                .min_by_key(|&(distance, _)| distance)
                .expect("unable to find nearest neighbour!");

            if nn.len() == 1 {
                let &centroid = nn.get(0).expect("our neighbour has gone fishing!");

                // increment area size
                *area.entry(centroid).or_insert(0) += 1;

                if x == min_x || y == min_y || x == max_x || y == max_y {
                    // mark centroid as unbounded
                    unbounded.insert(centroid);
                }
            };
        }
    }

    // find largest enclosed area
    area.into_iter()
        .filter(|(centroid, _)| !unbounded.contains(centroid))
        .map(|(_, area)| area)
        .max()
        .expect("unable to find largest enclosed area")
}

pub fn part2(input: &[Point], distance: usize) -> isize {
    let mut area = 0;

    // determine the min/max of X and Y
    let mut min_x = std::usize::MAX;
    let mut min_y = std::usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for point in input.into_iter() {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    // scan grid pattern from min X+Y to max X+Y
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let point = Point { x, y };

            // compute Manhatten distance against all neighbours
            let distances: usize = input
                .into_iter()
                .map(|neighbour| manhatten(&point, neighbour))
                .sum();

            if distances < distance {
                area += 1;
            }
        }
    }

    area
}

pub fn get_input(f: impl Read) -> Result<Vec<Point>, Box<dyn Error>> {
    // parse data from input.txt
    let re = Regex::new(r"^(\d+), (\d+)$")?;

    BufReader::new(f)
        .lines()
        .map(|line| -> Result<Option<Point>, Box<dyn Error>> {
            if let Some(parsed) = re.captures(line?.as_str()) {
                let try_parse = |n| -> Result<_, Box<dyn Error>> {
                    Ok(parsed
                        .get(n)
                        .ok_or_else(|| "malformed input")?
                        .as_str()
                        .parse()?)
                };
                Ok(Some(Point {
                    x: try_parse(1)?,
                    y: try_parse(2)?,
                }))
            } else {
                Ok(None)
            }
        })
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

    const DATA: &str = r#"
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
        "#;

    #[test]
    fn test_part1() {
        let input = get_input(DATA.as_bytes()).unwrap();
        assert_eq!(17, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input(DATA.as_bytes()).unwrap();
        assert_eq!(16, part2(&input, 32));
    }
}

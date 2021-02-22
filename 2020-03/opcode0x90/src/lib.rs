use std::{
    error::Error,
    io::{BufRead, BufReader, Read},
};

pub fn part1(input: &[String], dx: usize, dy: usize) -> usize {
    let mut count = 0;
    let mut x = 0;

    for line in input.iter().step_by(dy).skip(1) {
        x = (x + dx) % line.len();

        let c = line.chars().nth(x).unwrap();
        if c == '#' {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &[String]) -> usize {
    part1(&input, 1, 1)
        * part1(&input, 3, 1)
        * part1(&input, 5, 1)
        * part1(&input, 7, 1)
        * part1(&input, 1, 2)
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

    static DATA: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn test_part1() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part1(&input, 3, 1), 7);
    }

    #[test]
    fn test_part2() {
        let input = get_input(Cursor::new(DATA)).unwrap();

        assert_eq!(part2(&input), 336);
    }
}

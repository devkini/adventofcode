use std::{
    collections::BinaryHeap,
    error::Error,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BoardingPass {
    row: usize,
    column: usize,
    id: usize,
}

pub fn part1(input: &[BoardingPass]) -> usize {
    input.iter().map(|pass| pass.id).max().unwrap()
}

pub fn part2(input: &[BoardingPass]) -> usize {
    let mut seats = input.iter().map(|pass| pass.id).collect::<BinaryHeap<_>>();
    let mut last = seats.pop().unwrap();

    while let Some(id) = seats.pop() {
        if last - id > 1 {
            return id + 1;
        }
        last = id;
    }
    unreachable!()
}

impl FromStr for BoardingPass {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn bsp(s: &str) -> usize {
            let mut lower = 0;
            let mut upper = 2usize.pow(s.len() as u32) - 1;

            for c in s.chars() {
                match c {
                    'F' | 'L' => upper = lower + ((upper - lower) / 2),
                    'B' | 'R' => lower = lower + ((upper - lower) / 2) + 1,
                    _ => unreachable!(),
                }
            }
            assert_eq!(lower, upper);
            lower
        }

        // split line into tokens
        let (row_, column_) = s.split_at(7);
        let row = bsp(row_);
        let column = bsp(column_);

        Ok(BoardingPass {
            row,
            column,
            id: row * 8 + column,
        })
    }
}

pub fn get_input(f: impl Read) -> Result<Vec<BoardingPass>, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut buffer = vec![];

    // read the input line by line
    for line in reader.lines() {
        // attempt to parse each line
        let value = line?.parse::<BoardingPass>().unwrap();
        buffer.push(value);
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static DATA: &str = r#"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;

    #[test]
    fn test_input() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        let mut pass = input.into_iter();

        assert_eq!(
            Some(BoardingPass {
                row: 70,
                column: 7,
                id: 567
            }),
            pass.next()
        );
        assert_eq!(
            Some(BoardingPass {
                row: 14,
                column: 7,
                id: 119
            }),
            pass.next()
        );
        assert_eq!(
            Some(BoardingPass {
                row: 102,
                column: 4,
                id: 820
            }),
            pass.next()
        );
        assert_eq!(None, pass.next());
    }

    #[test]
    fn test_part1() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(820, part1(&input));
    }
}

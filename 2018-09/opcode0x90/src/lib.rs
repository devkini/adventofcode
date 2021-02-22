use std::collections::VecDeque;
use std::error::Error;
use std::io::{BufReader, Read};

use regex::Regex;

pub struct Config {
    players: usize,
    last_marble: usize,
}

pub fn part1(input: &Config) -> usize {
    /// rotate given VecDeque to the left in-place by given n times
    fn rotate_left<T>(deque: &mut VecDeque<T>, n: usize) {
        for _ in 0..n {
            if let Some(v) = deque.pop_front() {
                deque.push_back(v);
            }
        }
    }
    /// rotate given VecDeque to the right in-place by given n times
    fn rotate_right<T>(deque: &mut VecDeque<T>, n: usize) {
        for _ in 0..n {
            if let Some(v) = deque.pop_back() {
                deque.push_front(v);
            }
        }
    }

    let mut board = VecDeque::with_capacity(input.last_marble);
    let mut score = vec![0; input.players];

    // begin the simulation
    for i in 0..=input.last_marble {
        // is the current marble divisible by 23?
        if i == 0 || i % 23 != 0 {
            // rotate the board left twice
            rotate_left(&mut board, 2);

            // insert the new marble
            board.push_front(i);
        } else {
            let current_player = i % input.players;

            // player gets to keep the marble and also 7th marble to the left
            // println!("before rotate_right: {:?}", board);
            rotate_right(&mut board, 7);
            // println!("after rotate_right: {:?}", board);
            score[current_player] += i + board
                .pop_front()
                .expect("apparently we have lost our marbles!");
        }
    }
    // println!("board: {:?}", board);
    // println!("score: {:?}", score);

    // locate the high score
    score
        .into_iter()
        .max()
        .expect("unable to compute high score!")
}

pub fn part2(input: &Config) -> usize {
    // last_marble is 100x larger
    part1(&Config {
        players: input.players,
        last_marble: input.last_marble * 100,
    })
}

pub fn get_input(f: impl Read) -> Result<Config, Box<dyn Error>> {
    // read data from input.txt
    let mut buf = String::new();
    BufReader::new(f).read_to_string(&mut buf)?;

    // parse the input
    let re = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$")?;
    let parsed = re.captures(buf.trim()).ok_or_else(|| "malformed input")?;

    let try_parse = |n| -> Result<_, Box<dyn Error>> {
        Ok(parsed
            .get(n)
            .ok_or_else(|| "unable to parse input")?
            .as_str()
            .parse()?)
    };

    Ok(Config {
        players: try_parse(1)?,
        last_marble: try_parse(2)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            32,
            part1(
                &get_input("9 players; last marble is worth 25 points".as_bytes())
                    .expect("unable to parse input")
            )
        );
        assert_eq!(
            8317,
            part1(
                &get_input("10 players; last marble is worth 1618 points".as_bytes())
                    .expect("unable to parse input")
            )
        );
        assert_eq!(
            146373,
            part1(
                &get_input("13 players; last marble is worth 7999 points".as_bytes())
                    .expect("unable to parse input")
            )
        );
        assert_eq!(
            2764,
            part1(
                &get_input("17 players; last marble is worth 1104 points".as_bytes())
                    .expect("unable to parse input")
            )
        );
        assert_eq!(
            54718,
            part1(
                &get_input("21 players; last marble is worth 6111 points".as_bytes())
                    .expect("unable to parse input")
            )
        );
        assert_eq!(
            37305,
            part1(
                &get_input("30 players; last marble is worth 5807 points".as_bytes())
                    .expect("unable to parse input")
            )
        );
    }
}

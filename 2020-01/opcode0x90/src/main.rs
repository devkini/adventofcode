use std::{error::Error, fs::File};

use aoc::*;

fn main() -> Result<(), Box<dyn Error>> {
    // read input from input.txt
    let f = File::open("input.txt").expect("unable to read input.txt");
    let input = get_input(f)?;

    let part1 = part1(&input);
    println!("part1: {:?}", part1);

    let part2 = part2(&input);
    println!("part2: {}", part2);

    Ok(())
}

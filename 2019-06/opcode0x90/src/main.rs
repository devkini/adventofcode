use std::{
    fs::File,
    io::{BufReader, Read},
};

use aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read data from input.txt
    let f = File::open("input.txt").expect("input.txt not found!");

    // parse the input data
    let mut buf = String::new();
    BufReader::new(f).read_to_string(&mut buf)?;
    let input = parse_input(buf.as_str());

    let part1 = part1(&input);
    println!("part1: {}", part1);

    let part2 = part2(&input);
    println!("part2: {}", part2);

    Ok(())
}

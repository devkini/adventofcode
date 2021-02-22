use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    // read input from input.txt
    let f = File::open("input.txt").expect("unable to read input.txt");
    let input = get_input(f)?;

    let part1 = part1(&input);
    println!("part1: {}", part1);

    let part2 = part2(&input);
    println!("part2: {}", part2);

    Ok(())
}

fn get_input(f: impl Read) -> Result<Vec<usize>, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut buffer = vec![];

    // read the input line by line
    for line in reader.lines() {
        // attempt to parse each line into unsigned integer
        let value = line?.parse::<usize>()?;
        buffer.push(value);
    }

    Ok(buffer)
}

fn part1(input: &[usize]) -> usize {
    input.iter().map(|x| x / 3 - 2).sum()
}

fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .map(|&x| -> usize {
            let mut fuel = vec![];
            let mut remainder = x as isize;

            loop {
                remainder = remainder / 3 - 2;

                if remainder > 0 {
                    fuel.push(remainder as usize);
                } else {
                    break;
                }
            }

            fuel.into_iter().sum()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[14]), 2);
        assert_eq!(part1(&[1969]), 654);
        assert_eq!(part1(&[100_756]), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[14]), 2);
        assert_eq!(part2(&[1969]), 966);
        assert_eq!(part2(&[100_756]), 50346);
    }
}

use {
    itertools::Itertools,
    std::{
        error::Error,
        io::{BufRead, BufReader, Read},
    },
};

pub fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_combinations()
        .find(|(&a, &b)| a + b == 2020)
        .map(|(&a, &b)| a * b)
        .expect("unable to find combination of sum that amounts to 2020!")
}

pub fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_combinations::<(_, _, _)>()
        .find(|(&a, &b, &c)| a + b + c == 2020)
        .map(|(&a, &b, &c)| a * b * c)
        .expect("unable to find combination of sum that amounts to 2020!")
}

pub fn get_input(f: impl Read) -> Result<Vec<usize>, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}

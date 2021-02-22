use std::{collections::HashMap, iter::successors};

type Orbit<'a> = HashMap<&'a str, &'a str>;

//
// all credit goes to https://github.com/mitsuhiko/aoc19/blob/master/aoc6/src/main.rs
//
fn walk<'a>(orbit: &'a Orbit, to: &'a str) -> impl Iterator<Item = &'a str> {
    successors(Some(to), move |&to| orbit.get(to).copied()).skip(1)
}

pub fn part1(input: &Orbit) -> usize {
    input.keys().fold(0, |x, to| x + walk(input, to).count())
}

pub fn part2(input: &Orbit) -> usize {
    let distance = move |a, b| -> Option<usize> {
        // start a trace from destination going backwards
        let b_orbits: Vec<_> = walk(input, b).collect();

        // start a trace from source, and try to meet in the middle with destination trace
        walk(input, a)
            .enumerate()
            .filter_map(|(d, p)| Some(d + b_orbits.iter().rposition(|&x| x == p)?))
            .next()
    };
    distance("YOU", "SAN").unwrap()
}

pub fn parse_input(s: &str) -> Orbit<'_> {
    // build the reversed adjacency list
    s.trim()
        .lines()
        .filter_map(|line| Some((line.find(')')?, line)))
        .map(|(i, line)| (&line[i + 1..], &line[..i]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            42,
            part1(&parse_input(
                r#"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
        "#
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            4,
            part2(&parse_input(
                r#"
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
        "#,
            ))
        );
    }
}

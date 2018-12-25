use std::error::Error;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct License {
    children: Vec<License>,
    metadata: Vec<usize>,
}

pub fn part1(input: &License) -> usize {
    // recursively fold the tree
    fn checksum(license: &License) -> usize {
        let a = license.metadata.iter().sum::<usize>();
        let b = license.children.iter().map(checksum).sum::<usize>();
        a + b
    }
    checksum(input)
}

pub fn part2(input: &License) -> usize {
    // recursively fold the tree
    fn checksum(license: &License) -> usize {
        // do we have children?
        if license.children.is_empty() {
            // value is the sum of its metadata entries
            license.metadata.iter().sum::<usize>()
        } else {
            // value is the sum of the values of the child nodes referenced
            // by the metadata entries
            license
                .metadata
                .iter()
                .map(|&i| -> usize {
                    // our vector is zero-indexed
                    let i = i - 1;

                    // if a referenced child node does not exist, skip it
                    if let Some(child) = license.children.get(i) {
                        // compute the value of child node
                        checksum(child)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        }
    }
    checksum(input)
}

pub fn get_input(f: impl Read) -> Result<License, Box<dyn Error>> {
    // parse input from input.txt
    let input = BufReader::new(f)
        .split(b' ')
        .map(|line| -> Result<usize, Box<dyn Error>> {
            // collect the chars into UTF-8, then parse into integer
            Ok(String::from_utf8(line?)?.trim().parse::<usize>()?)
        })
        .collect::<Result<Vec<_>, _>>()?;

    fn extract_license(it: &mut impl Iterator<Item = usize>) -> License {
        let mut license = License::default();

        if let (Some(n_children), Some(n_metadata)) = (it.next(), it.next()) {
            // parse all child nodes
            for _ in 0..n_children {
                // recursively extract the child nodes
                license.children.push(extract_license(it));
            }
            // extract the rest of metadata
            license.metadata.extend(it.take(n_metadata));
        }
        license
    }

    Ok(extract_license(&mut input.into_iter()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"#;

    #[test]
    fn test_part1() {
        let input = get_input(DATA.as_bytes()).expect("unable to parse input");
        assert_eq!(138, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = get_input(DATA.as_bytes()).expect("unable to parse input");
        assert_eq!(66, part2(&input));
    }
}

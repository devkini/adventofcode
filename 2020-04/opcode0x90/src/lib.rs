#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    io::{BufRead, BufReader, Read},
    iter::FromIterator,
};

type Passport = HashMap<String, String>;

lazy_static! {
    static ref REQUIRED_KEYS: HashSet<String> = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
        .iter()
        .cloned()
        .map(str::to_owned)
        .collect();
}
pub fn part1(input: &[Passport]) -> usize {
    input
        .iter()
        .map(|passport| passport.keys().cloned())
        .map(HashSet::from_iter)
        .filter(|keys| keys.is_superset(&REQUIRED_KEYS))
        .count()
}

lazy_static! {
    static ref VALID_ECL: HashSet<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .map(str::to_owned)
        .collect();
}
pub fn part2(input: &[Passport]) -> usize {
    fn valid_passport(passport: &Passport) -> bool {
        let byr = passport
            .get("byr")
            .and_then(|s| s.parse::<usize>().ok())
            .map_or(false, |v| (1920..=2002).contains(&v));
        let iyr = passport
            .get("iyr")
            .and_then(|s| s.parse::<usize>().ok())
            .map_or(false, |v| (2010..=2020).contains(&v));
        let eyr = passport
            .get("eyr")
            .and_then(|s| s.parse::<usize>().ok())
            .map_or(false, |v| (2020..=2030).contains(&v));
        let hgt = passport
            .get("hgt")
            .and_then(|s| s.find(char::is_alphabetic).map(|i| s.split_at(i)))
            .and_then(|(s, unit)| s.parse::<usize>().ok().map(|v| (v, unit)))
            .map_or(false, |(v, unit)| match unit {
                "cm" => (150..=193).contains(&v),
                "in" => (59..=76).contains(&v),
                _ => false,
            });
        let hcl = passport.get("hcl").map_or(false, |s| {
            s.len() == 7 && s.starts_with('#') && s.chars().skip(1).all(|c| c.is_digit(16))
        });
        let ecl = passport.get("ecl").map_or(false, |s| VALID_ECL.contains(s));
        let pid = passport
            .get("pid")
            .map_or(false, |s| s.len() == 9 && s.chars().all(char::is_numeric));

        byr && iyr && eyr && hgt && hcl && ecl && pid
    }

    input.iter().cloned().filter(valid_passport).count()
}

pub fn get_input(f: impl Read) -> Result<Vec<Passport>, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    lines.push("".to_owned());

    let mut tokens = vec![];
    let mut result = vec![];

    for line in lines.into_iter() {
        if !line.is_empty() {
            tokens.extend(
                line.trim()
                    // buf is a string containing key:value pairs delimited by whitespace
                    .split_whitespace()
                    // parse key:value pair
                    .map(|token| token.split_at(token.find(':').unwrap()))
                    .map(|(a, b)| (a.to_owned(), b.get(1..).unwrap().to_owned())),
            );
        } else if !tokens.is_empty() {
            // parse the tokens into HashMap
            let passport: Passport = HashMap::from_iter(tokens.drain(..));
            result.push(passport);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static DATA: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;

    #[test]
    fn test_input() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(input.len(), 4);
    }

    #[test]
    fn test_part1() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let invalid_input = get_input(Cursor::new(
            r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#,
        ))
        .unwrap();
        assert_eq!(part2(&invalid_input), 0);

        let valid_input = get_input(Cursor::new(
            r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#,
        ))
        .unwrap();
        assert_eq!(part2(&valid_input), 4);
    }
}

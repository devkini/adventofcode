use std::{
    collections::HashSet,
    error::Error,
    io::{BufRead, BufReader, Read},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    op: String,
    imm1: isize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    buffer: Vec<Instruction>,

    pc: usize,
    acc: isize,
}

fn run_until_halt(m: &mut Machine) -> bool {
    let mut visited = HashSet::<usize>::new();

    loop {
        m.step();

        if m.pc == m.buffer.len() {
            return true;
        } else if !visited.insert(m.pc) {
            return false;
        }
    }
}

pub fn part1(input: &[Instruction]) -> isize {
    let mut m = Machine::new(input);
    run_until_halt(&mut m);
    m.acc
}

pub fn part2(input: &[Instruction]) -> isize {
    for (pc, inst) in input.iter().enumerate() {
        if inst.op != "acc" {
            let op = if inst.op == "nop" { "jmp" } else { "nop" };

            let mut m = Machine::new(input);

            // patch the instruction
            m.patch(pc, op.to_owned());

            // lets see if it works
            if run_until_halt(&mut m) {
                return m.acc;
            }
            // println!();
        }
    }

    unreachable!()
}

pub fn get_input(f: impl Read) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut buffer = vec![];

    // read the input line by line
    for line in reader.lines() {
        // attempt to parse each line
        let value = line?.parse::<Instruction>()?;
        buffer.push(value);
    }

    Ok(buffer)
}

impl Machine {
    fn new(buffer: &[Instruction]) -> Machine {
        Machine {
            buffer: buffer.to_vec(),
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        if let Some(inst) = self.buffer.get(self.pc) {
            let offset = match inst.op.as_str() {
                "nop" => 1,
                "acc" => {
                    self.acc += inst.imm1;
                    1
                }
                "jmp" => inst.imm1,
                _ => unreachable!(),
            };
            self.pc = (self.pc as isize + offset) as usize;

            // println!("{:?} => pc: {}, acc: {}", inst, self.pc, self.acc);
        }
    }

    fn patch(&mut self, pc: usize, op: String) {
        self.buffer[pc].op = op;
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let op = tokens.next().expect("invalid instruction!");
        let imm1 = tokens.next().expect("missing operand!");

        Ok(Instruction {
            op: op.to_owned(),
            imm1: imm1.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static DATA: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn test_input() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(input.len(), 9);
    }

    #[test]
    fn test_part1() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = get_input(Cursor::new(DATA)).unwrap();
        assert_eq!(part2(&input), 8);
    }
}

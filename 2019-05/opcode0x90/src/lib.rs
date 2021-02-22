use std::{
    error::Error,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Clone)]
pub struct Program {
    // program buffer
    buffer: Vec<isize>,

    // program states
    ip: usize,
    output: isize,
}

#[derive(Debug, Clone)]
enum Instruction {
    Add(isize, isize, usize),
    Multiply(isize, isize, usize),
    Input(usize),
    Output(isize),
    JumpIfTrue(isize, usize),
    JumpIfFalse(isize, usize),
    LessThan(isize, isize, usize),
    Equals(isize, isize, usize),
    Halt,
}

impl Program {
    // dereference the value if position mode
    fn dereference(&self, addr: isize, imm: bool) -> isize {
        if imm {
            addr
        } else {
            // println!(
            //     "| dereference: {:?} ({:?})",
            //     addr,
            //     &self.buffer[(addr - 2) as usize..=(addr + 2) as usize]
            // );
            self.buffer[addr as usize]
        }
    }
    // decode the instruction at instruction pointer
    fn decode(&self) -> Option<Instruction> {
        // println!("decode: {:?}", self);

        // decode the opcode at instruction pointer
        let raw = self.buffer[self.ip] as usize;
        let opcode = raw % 100;
        // println!("| scope: {:?}", &self.buffer[self.ip..]);
        // println!("| opcode: {:?}", opcode);

        let decoded = match opcode {
            99 => Instruction::Halt,
            // 1 param instruction
            3 => Instruction::Input(self.buffer[self.ip + 1] as usize),
            4 => {
                // decode addressing mode
                let imm1 = ((raw % 1000) - (raw % 100)) != 0;

                // dereference the values, if necessary
                let a = self.buffer[self.ip + 1];
                let val = self.dereference(a, imm1);

                Instruction::Output(val)
            }
            // 2 params instruction
            5..=6 => {
                // decode addressing mode
                let imm1 = ((raw % 1000) - (raw % 100)) != 0;
                let imm2 = ((raw % 10000) - (raw % 1000)) != 0;
                // println!("| mode: {:?}, {:?}", imm1, imm2);

                if let [cmp, ip] = self.buffer[self.ip + 1..=self.ip + 2] {
                    // dereference the values, if necessary
                    let val = self.dereference(cmp, imm1);
                    let ip = self.dereference(ip, imm2) as usize;
                    // println!("| val1, dst: {:?}, {:?}", val1, dst);

                    // return decoded instruction
                    match opcode {
                        5 => Instruction::JumpIfTrue(val, ip),
                        6 => Instruction::JumpIfFalse(val, ip),
                        _ => unreachable!(), // this is not supposed to happen
                    }
                } else {
                    // this is not supposed to happen
                    unreachable!();
                }
            }
            // 3 params instruction
            1..=2 | 7..=8 => {
                // decode addressing mode
                let imm1 = ((raw % 1000) - (raw % 100)) != 0;
                let imm2 = ((raw % 10000) - (raw % 1000)) != 0;
                let imm3 = ((raw % 100_000) - (raw % 10000)) != 0;
                // println!("| mode: {:?}, {:?}, {:?}", imm1, imm2, imm3);

                // mode3 cannot be immediate mode
                assert!(!imm3);

                if let [a, b, dst] = self.buffer[self.ip + 1..=self.ip + 3] {
                    // dereference the values, if necessary
                    let val1 = self.dereference(a, imm1);
                    let val2 = self.dereference(b, imm2);
                    let dst = dst as usize;
                    // println!("| val1, val2, dst: {:?}, {:?}, {:?}", val1, val2, dst);

                    // return decoded instruction
                    match opcode {
                        1 => Instruction::Add(val1, val2, dst),
                        2 => Instruction::Multiply(val1, val2, dst),
                        7 => Instruction::LessThan(val1, val2, dst),
                        8 => Instruction::Equals(val1, val2, dst),
                        _ => unreachable!(), // this is not supposed to happen
                    }
                } else {
                    // this is not supposed to happen
                    unreachable!();
                }
            }
            // illegal opcode
            _ => {
                println!("illegal opcode: {:?}", opcode);
                return None;
            }
        };
        Some(decoded)
    }

    // execute instruction at current instruction pointer
    fn execute(&mut self, instruction: Instruction, input: isize) -> usize {
        match instruction {
            Instruction::Add(a, b, dst) => {
                // add operand and store result at [dst]
                self.buffer[dst] = a + b;
                4
            }
            Instruction::Multiply(a, b, dst) => {
                //  multiply operand and store result at [dst]
                self.buffer[dst] = a * b;
                4
            }
            Instruction::Input(dst) => {
                // store input at [dst]
                self.buffer[dst] = input;
                2
            }
            Instruction::Output(val) => {
                // store output in program state
                self.output = val;
                2
            }
            Instruction::JumpIfTrue(val, ip) => {
                // if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter
                if val != 0 {
                    self.ip = ip;
                    0
                } else {
                    3
                }
            }
            Instruction::JumpIfFalse(val, ip) => {
                // if the first parameter is zero, it sets the instruction pointer to the value from the second parameter
                if val == 0 {
                    self.ip = ip;
                    0
                } else {
                    3
                }
            }
            Instruction::LessThan(a, b, dst) => {
                let val = (a < b) as isize;
                // println!("val: {:?}", val);
                self.buffer[dst] = val;
                4
            }
            Instruction::Equals(a, b, dst) => {
                let val = (a == b) as isize;
                // println!("val: {:?}", val);
                self.buffer[dst] = val;
                4
            }
            Instruction::Halt => 0,        // halt execution
            _ => panic!("unknown opcode"), // this is not supposed to happen
        }
    }

    // run program till halt
    fn run(&mut self, input: isize) -> isize {
        // cap execution at 1000 iterations to avoid possible infinite loop
        for _ in 1..1000 {
            // decode the instruction
            if let Some(instruction) = self.decode() {
                // execute the instruction and return instruction size
                // println!("{:?}", instruction);
                let size = self.execute(instruction.clone(), input);

                match instruction {
                    Instruction::Output(_) => {
                        // println!("-> {:?}", self.output);
                        // assert!(self.output == 0);
                    }
                    Instruction::Halt => return self.output, // execution halted, return most recent output
                    _ => (),
                }

                // move to next instruction
                self.ip += size;
            } else {
                // unable to decode instruction
                panic!("illegal instruction");
            }
        }

        // program is stuck in loop
        unreachable!();
    }
}

pub fn part1(input: &Program) -> isize {
    // make a copy of input so we can freely mutate it
    let mut program = input.clone();

    // run program until halt
    program.run(1)
}

pub fn part2(input: &Program) -> isize {
    // make a copy of input so we can freely mutate it
    let mut program = input.clone();

    // run program until halt
    program.run(5)
}

pub fn get_input(f: impl Read) -> Result<Program, Box<dyn Error>> {
    let reader = BufReader::new(f);
    let mut buffer = vec![];

    // tokenize the file delimited by comma
    for token in reader.split(b',') {
        // parse the token into UTF-8 string
        let s = String::from_utf8(token?)?;
        let s = s.trim(); // lifetime issue, just shadow the original variable

        if !s.is_empty() {
            // attempt to parse the token into unsigned integer
            let value = s.parse::<isize>()?;

            // add value to buffer
            buffer.push(value);
        }
    }

    Ok(Program {
        buffer,
        ip: 0,
        output: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() {
        get_input(Cursor::new("3,0,4,0,99")).unwrap().run(1);
        get_input(Cursor::new("1002,4,3,4,33")).unwrap().run(1);
        get_input(Cursor::new("1101,100,-1,4,0")).unwrap().run(1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            1,
            get_input(Cursor::new("3,9,8,9,10,9,4,9,99,-1,8"))
                .unwrap()
                .run(8)
        );
        assert_eq!(
            0,
            get_input(Cursor::new("3,9,8,9,10,9,4,9,99,-1,8"))
                .unwrap()
                .run(9)
        );

        assert_eq!(
            1,
            get_input(Cursor::new("3,9,7,9,10,9,4,9,99,-1,8"))
                .unwrap()
                .run(-7)
        );
        assert_eq!(
            0,
            get_input(Cursor::new("3,9,7,9,10,9,4,9,99,-1,8"))
                .unwrap()
                .run(8)
        );

        assert_eq!(
            1,
            get_input(Cursor::new("3,3,1108,-1,8,3,4,3,99"))
                .unwrap()
                .run(8)
        );
        assert_eq!(
            1,
            get_input(Cursor::new("3,3,1107,-1,8,3,4,3,99"))
                .unwrap()
                .run(-7)
        );

        assert_eq!(
            1,
            get_input(Cursor::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"))
                .unwrap()
                .run(-7)
        );
        assert_eq!(
            1,
            get_input(Cursor::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"))
                .unwrap()
                .run(-7)
        );

        assert_eq!(
            999,
            get_input(Cursor::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"))
            .unwrap()
            .run(-7)
        );
        assert_eq!(
            1000,
            get_input(Cursor::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"))
            .unwrap()
            .run(8)
        );
        assert_eq!(
            1001,
            get_input(Cursor::new("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"))
            .unwrap()
            .run(9999)
        );
    }
}

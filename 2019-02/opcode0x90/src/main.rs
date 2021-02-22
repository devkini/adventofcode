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

    // tokenize the file delimited by comma
    for token in reader.split(b',') {
        // parse the token into UTF-8 string
        let s = String::from_utf8(token?)?;
        let s = s.trim(); // lifetime issue, just shadow the original variable

        if !s.is_empty() {
            // attempt to parse the token into unsigned integer
            let value = s.parse::<usize>()?;

            // add value to buffer
            buffer.push(value);
        }
    }

    Ok(buffer)
}

fn part1(input: &[usize]) -> usize {
    // patch the input before running the program
    let mut buf = input.to_vec();
    buf[1] = 12;
    buf[2] = 2;

    // now run it
    interpret(&buf).expect("program has crashed!")
}

fn part2(input: &[usize]) -> usize {
    // search for correct noun and verb (capped to 1000 iterations to avoid infinite loop)
    for noun in 0..1000 {
        for verb in 0..1000 {
            // println!("noun: {:?}, verb: {:?}", noun, verb);

            // patch the input before running the program
            let mut buf = input.to_vec();
            buf[1] = noun;
            buf[2] = verb;

            // now run it (ignoring all error)
            if let Ok(result) = interpret(&buf) {
                if result == 19_690_720 {
                    // found the result
                    return 100 * noun + verb;
                }
            }
        }
    }

    // failed to determine the noun and verb
    unreachable!();
}

fn interpret(input: &[usize]) -> Result<usize, Box<dyn Error>> {
    // make a copy of input so we can freely mutate it
    let mut buf = input.to_vec();

    // run the program (capped to 1000 iterations to avoid infinite loop)
    let mut pc = 0;

    for _ in 0..1000 {
        // println!("buffer: {:?}", buf);

        // read the opcode at pc
        let opcode = buf[pc];
        // println!("opcode: {:?}", opcode);

        match opcode {
            99 => {
                // halt, we're done here!
                // println!("*** halt! ***");
                return Ok(buf[0]);
            }
            1..=2 => {
                // refine the match now we know we can safely consume 4 bytes
                if let [a, b, dst] = buf[pc + 1..pc + 4] {
                    // dereference the pointer
                    let val1 = buf.get(a).ok_or("program has failed")?;
                    let val2 = buf.get(b).ok_or("program has failed")?;

                    match opcode {
                        1 => buf[dst] = val1 + val2, // add
                        2 => buf[dst] = val1 * val2, // multiply
                        _ => unreachable!(),         // this is not supposed to happen
                    }
                } else {
                    // this is not supposed to happen
                    unreachable!();
                }
            }
            _ => unreachable!(), // illegal opcode
        }

        // move to next instruction
        pc += 4;
    }

    // hold on, you're not supposed to be here!
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() {
        assert_eq!(
            interpret(&get_input(Cursor::new("1,9,10,3,2,3,11,0,99,30,40,50")).unwrap()).unwrap(),
            3500
        );
        assert_eq!(
            interpret(&get_input(Cursor::new("1,0,0,0,99")).unwrap()).unwrap(),
            2
        );
        assert_eq!(
            interpret(&get_input(Cursor::new("2,3,0,3,99")).unwrap()).unwrap(),
            2
        );
        assert_eq!(
            interpret(&get_input(Cursor::new("2,4,4,5,99,0")).unwrap()).unwrap(),
            2
        );
        assert_eq!(
            interpret(&get_input(Cursor::new("1,1,1,4,99,5,6,0,99")).unwrap()).unwrap(),
            30
        );
    }

    #[test]
    fn test_part2() {
        //
    }
}

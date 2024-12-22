pub enum OpKind {
    Literal,
    Combo,
}

// adv 3 -> a
// out 4 = out a
// jump 0
// halt

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Op {
    adv, // 0: a / (pow**combo) -> a
    bxl, // 1: b^op
    bst, // 2: combo % 7 -> b
    jnz, // 3: jump non-zero op
    bxc, // 4: b^c -> b
    out, // 5: combo % 8 -> output
    bdv, // 6: a / (pow**combo) -> b
    cdv, // 7: a / (pow**combo) -> c
}

pub struct Program {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub program: Vec<u8>, // actually just 0-7
    pub instruction_pointer: usize,
}

pub fn parse_input(input: &str) -> Program {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    lines.next(); // blank

    let opcodes: Vec<_> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    Program {
        a,
        b,
        c,
        program: opcodes,
        instruction_pointer: 0,
    }
}

impl Program {
    pub fn run(&mut self) -> Vec<u64> {
        let mut output: Vec<_> = Vec::new();
        loop {
            let instruction = self.program.get(self.instruction_pointer);
            // eprintln!("instruction: {instruction:?}");
            if instruction.is_none() {
                break;
            }

            let op = match instruction.unwrap() {
                0 => Op::adv,
                1 => Op::bxl,
                2 => Op::bst,
                3 => Op::jnz,
                4 => Op::bxc,
                5 => Op::out,
                6 => Op::bdv,
                7 => Op::cdv,
                _ => panic!("Invalid opcode!"),
            };
            let operand = self.program[self.instruction_pointer + 1];

            match op {
                Op::adv => {
                    self.a /= 2_u64.pow(self.combo(operand.into()) as u32);
                }
                Op::bxl => {
                    self.b ^= operand as u64;
                }
                Op::bst => {
                    self.b = self.combo(operand.into()) % 8;
                }
                Op::jnz => {
                    if self.a > 0 {
                        self.instruction_pointer = operand as usize;
                        // skip incrementing it below
                        continue;
                    }
                }
                Op::bxc => {
                    self.b ^= self.c;
                }
                Op::out => {
                    output.push(self.combo(operand.into()) % 8);
                }
                Op::bdv => {
                    self.b = self.a / 2_u64.pow(self.combo(operand.into()) as u32);
                }
                Op::cdv => {
                    self.c = self.a / 2_u64.pow(self.combo(operand.into()) as u32);
                }
            }

            self.instruction_pointer += 2;
        }
        output
    }

    pub fn combo(&self, operand: u64) -> u64 {
        if (0..=3).contains(&operand) {
            operand
        } else if operand == 4 {
            self.a
        } else if operand == 5 {
            self.b
        } else if operand == 6 {
            self.c
        } else if operand == 7 {
            panic!("Reserved")
        } else {
            panic!("Invalid operand.")
        }
    }
}

pub fn main(input: &str) -> String {
    let mut program = parse_input(input);
    let output = program.run();
    let x: Vec<_> = output.iter().map(|x| x.to_string()).collect();
    x.join(",")
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_example_1() {
        let mut state = parse_input(INPUT);
        assert_eq!(state.a, 729);
        assert_eq!(state.b, 0);
        assert_eq!(state.c, 0);
        assert_eq!(state.program, vec![0, 1, 5, 4, 3, 0]);

        let output = state.run();
        assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}

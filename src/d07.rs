use std::vec;

use itertools::{self, Itertools};

#[derive(PartialEq, Eq, Debug)]
pub struct Line {
    pub result: u64,
    pub operands: Vec<u64>,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Mul,
}

pub fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input.lines() {
        let (result, operands_raw) = line.split_once(": ").expect("Missing ':'");
        // eprintln!("result={result}");
        lines.push(Line {
            result: result.parse().unwrap(),
            operands: operands_raw
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect(),
        });
    }
    lines
}

impl Line {
    pub fn find_solutions(&self) -> bool {
        // let mut result = Vec::new();
        let n_operators = self.operands.len() - 1;
        let options = [Operator::Add, Operator::Mul];

        // [ [Add], [Mul] ]
        // [ [Add, Add], [Add, Mul], [Mul, Add], [Mul, Mul]]
        // [ [Add, Add, Add], [Add, Add, Mul], [Add, Mul, Mul], ... [Mul, Mul, Mul]]
        // ...

        for operators in vec![&options; n_operators]
            .iter()
            .map(|x| x.iter())
            .multi_cartesian_product()
        {
            let mut acc = 0;
            acc += self.operands[0];

            for (operand, operator) in self.operands[1..].iter().zip(&operators) {
                match operator {
                    Operator::Add => acc += operand,
                    Operator::Mul => acc *= operand,
                }

                if acc > self.result {
                    break;
                }
            }

            if acc == self.result {
                // eprintln!("Valid! {:?}, permutation={:?}", self, operators);
                return true;
            }
        }
        false
    }
}

pub fn sum_valid_lines(lines: &[Line]) -> u64 {
    lines
        .iter()
        .filter(|line| line.find_solutions())
        .map(|line| line.result)
        .sum()
}

pub fn total_calibration_result(input: &str) -> u64 {
    let lines = parse_input(input);
    sum_valid_lines(&lines)
}

#[cfg(test)]
mod tests {
    use crate::d07::Line;

    use super::{parse_input, sum_valid_lines};

    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn test_parse_input() {
        let result = parse_input(INPUT);
        assert_eq!(
            result[0],
            Line {
                result: 190,
                operands: vec![10, 19]
            }
        );
    }

    #[test]
    fn test_example_1() {
        let data = parse_input(INPUT);
        let result = sum_valid_lines(&data);
        assert_eq!(result, 3749);
    }
}

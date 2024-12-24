use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

/*
Combine bits for wires starting with `z`.
*/

type Operation = (String, Op, String, String);

#[derive(Clone)]
pub enum Op {
    AND,
    OR,
    XOR,
}

pub fn parse_input(input: &str) -> (HashMap<String, u8>, Vec<Operation>) {
    let mut initial: HashMap<String, u8> = HashMap::new();
    let mut operations: Vec<_> = Vec::new();
    let mut initial_lines = true;

    for line in input.lines() {
        if line.is_empty() {
            initial_lines = false;
            continue;
        }
        if initial_lines {
            let (key, value) = line.split_once(": ").unwrap();
            initial.insert(key.to_string(), value.parse().unwrap());
        } else {
            let (inputs_raw, output) = line.split_once(" -> ").unwrap();
            let inputs = inputs_raw.split_whitespace().collect::<Vec<_>>();
            assert_eq!(inputs.len(), 3);
            let a = inputs[0].to_string();
            let b = inputs[2].to_string();
            let op = match inputs[1] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Unknown operation"),
            };

            operations.push((a, op, b, output.to_string()));
        }
    }
    (initial, operations)
}

pub fn run(initial: &HashMap<String, u8>, operations: &Vec<Operation>) -> HashMap<String, u8> {
    // We need to figure out which operations are ready to be run (i.e. both
    // their inputs are known)

    let mut ready = initial.clone();
    let mut stack: VecDeque<Operation> = VecDeque::with_capacity(operations.len());

    for op in operations.iter() {
        stack.push_back(op.clone());
    }

    while !stack.is_empty() {
        let (a, op, b, output) = stack.pop_front().unwrap();
        match (ready.get(&a), ready.get(&b)) {
            (Some(a_value), Some(b_value)) => {
                let result = match op {
                    Op::AND => a_value & b_value,
                    Op::OR => a_value | b_value,
                    Op::XOR => a_value ^ b_value,
                };
                ready.insert(output.to_string(), result);
            }
            _ => {
                stack.push_back((a, op, b, output));
            }
        }
    }

    // let mut wires = initial.clone();
    // let mut operations = operations.clone();

    // while !operations.is_empty() {
    //     let mut i = 0;
    //     while i < operations.len() {
    //         let (a, op, b, output) = operations[i].clone();
    //         let a_value = match a.parse::<u8>() {
    //             Ok(value) => value,
    //             Err(_) => {
    //                 if let Some(value) = wires.get(&a) {
    //                     *value
    //                 } else {
    //                     continue;
    //                 }
    //             }
    //         };
    //         let b_value = match b.parse::<u8>() {
    //             Ok(value) => value,
    //             Err(_) => {
    //                 if let Some(value) = wires.get(&b) {
    //                     *value
    //                 } else {
    //                     continue;
    //                 }
    //             }
    //         };

    //         let result = match op {
    //             Op::AND => a_value & b_value,
    //             Op::OR => a_value | b_value,
    //             Op::XOR => a_value ^ b_value,
    //         };

    //         wires.insert(output, result);
    //         operations.remove(i);
    //     }
    // }
    ready
}

pub fn main(input: &str) -> u64 {
    let (initial, operations) = parse_input(input);
    let result = run(&initial, &operations);
    let zs: String = result
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by(|(a, _), (b, _)| b.cmp(a))
        .map(|(_, v)| v.to_string())
        .collect();
    // eprintln!("zs={zs:?}");
    u64::from_str_radix(&zs, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    const INPUT2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_example_1() {
        let result = main(INPUT);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_2() {
        let result = main(INPUT2);
        assert_eq!(result, 2024);
    }

}

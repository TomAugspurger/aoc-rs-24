use std::ops::BitXor;

pub fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn simulate(number: u64, n_iterations: usize) -> u64 {
    let mut new_number = number;

    for _ in 0..n_iterations {
        let r1 = new_number * 64;

        new_number = new_number.bitxor(r1) % 16777216;

        let r2 = new_number / 32;
        new_number = new_number.bitxor(r2) % 16777216;

        let r3 = new_number * 2048;
        new_number = new_number.bitxor(r3) % 16777216;
    }

    new_number
}

pub fn main(input: &str) -> u64 {
    let numbers = parse_input(input);
    let result = numbers
        .iter()
        .map(|&n| simulate(n, 2000))
        // .inspect(|x| eprintln!("{x}"))
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1
10
100
2024";

    #[test]
    fn test_example_1() {
        let result = main(INPUT);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_simulate() {
        assert_eq!(simulate(123, 1), 15887950);
        assert_eq!(simulate(123, 2), 16495136);
    }
}

pub fn parse_input(input: &str) -> Vec<u64> {
    input.split(" ").map(|x| x.parse().unwrap()).collect()
}

pub fn blink(stones: &mut Vec<u64>) {
    let n = stones.len();  // the *original* length
    let mut i = 0;  // the *original* stone index we're processing
    let mut offset = 0;
    while i < n {
        let index = i + offset;
        let stone = stones[index];
        if stone == 0 {
            stones[index] = 1
        }

        else {
            let n_digits = stone.ilog10() + 1;
            if n_digits % 2 == 0 {
                // even number of digits
                // left is the first half of the digits
                let divisor = 10u64.pow(n_digits / 2);
                let left = stone / divisor;
                // right is the second half of the digits
                let right = stone % divisor;
                stones[index] = left;
                stones.insert(index + 1, right);
                offset += 1;
            } else {
                stones[index] *= 2024
            }

        }

        i += 1;

    }

}

pub fn main(input: &str) -> usize {
    let mut data = parse_input(input);
    for _i in 0..25 {
        blink(&mut data);
        // eprintln!("i={_i} stones={data:?}")
    }

    data.len()

}


#[cfg(test)]
mod tests {
    use crate::d11::main;

    const INPUT: &str = "125 17";

    #[test]
    fn test_example_1() {
        let result = main(INPUT);
        assert_eq!(result, 55312);
    }
}

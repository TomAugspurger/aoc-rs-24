use regex::Regex;

pub fn evaluate(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let left: &i32 = &caps[1].parse().unwrap();
            let right: &i32 = &caps[2].parse().unwrap();
            left * right
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::d03::evaluate;

    #[test]
    fn test_part_1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        // 2*4 + 5*5 + 11*8 + 8*5
        let result = evaluate(input);
        assert_eq!(result, 161);
    }
}

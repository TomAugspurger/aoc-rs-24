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

pub fn evaluate_switches(input: &str) -> i32 {
    let parts: Vec<_> = input.split("do()").collect();
    // each part in `parts` now has an implicit `do()` at the start,
    // meaning it's safe to sum *up to a `don't()`*.
    // So let's go through each part, split at don't(), grab the
    // first sub-part, and eval those.
    parts
        .iter()
        .map(|part| part.split_once("don't()").unwrap_or((part, part)).0)
        .map(evaluate)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::d03::{evaluate, evaluate_switches};

    #[test]
    fn test_part_1_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        // 2*4 + 5*5 + 11*8 + 8*5
        let result = evaluate(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = evaluate_switches(input);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_part_2_example2() {
        let input = "mul(2,4)don't()mul(5,5)do()mul(2,4)don't()mul(5,5)";
        let result = evaluate_switches(input);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_2_double_dont() {
        let input = "mul(2,4)don't()mul(5,5)don't()mul(5,5)do()mul(2,4)don't()mul(5,5)";

        /*


        */

        let result = evaluate_switches(input);
        assert_eq!(result, 16);
    }
}

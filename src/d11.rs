use std::collections::HashMap;

pub fn parse_input(input: &str) -> HashMap<u64, u64> {
    input.split(" ").map(|x| (x.parse().unwrap(), 1)).collect()
}

// pub fn blink(stones: &mut Vec<u64>) {
//     let n = stones.len(); // the *original* length
//     let mut i = 0; // the *original* stone index we're processing
//     let mut offset = 0;
//     while i < n {
//         let index = i + offset;
//         let stone = stones[index];
//         if stone == 0 {
//             stones[index] = 1
//         } else {
//             let n_digits = stone.ilog10() + 1;
//             if n_digits % 2 == 0 {
//                 // even number of digits
//                 // left is the first half of the digits
//                 let divisor = 10u64.pow(n_digits / 2);
//                 let left: u64 = stone / divisor;
//                 // right is the second half of the digits
//                 let right = stone % divisor;
//                 stones[index] = left;
//                 stones.insert(index + 1, right);
//                 offset += 1;
//             } else {
//                 stones[index] *= 2024
//             }
//         }

//         i += 1;
//     }
// }

/*
Can't use a List<int> for large number of blinks.

The transformation doesn't depend on the index, so let's
avoid it!

*/

pub fn blink(counts: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new = HashMap::with_capacity(counts.capacity());

    for (k, v) in counts.iter() {
        let (left, maybe_right) = transform(*k);

        // If I have 2 '2's, then the output should have 2 '2048's

        // Move v "copies" of k to the new one stone's value.
        *new.entry(left).or_insert(0) += v;

        if let Some(right) = maybe_right {
            *new.entry(right).or_insert(0) += v;
        }
    }

    new
}

pub fn transform(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        (1, None)
    } else {
        let n_digits = stone.ilog10() + 1;
        if n_digits % 2 == 0 {
            // even number of digits
            // left is the first half of the digits
            let divisor = 10u64.pow(n_digits / 2);
            let left: u64 = stone / divisor;
            // right is the second half of the digits
            let right = stone % divisor;
            (left, Some(right))
        } else {
            (stone * 2024, None)
        }
    }
}

pub fn main(input: &str, n_blinks: u64) -> u64 {
    let mut stones = parse_input(input);
    for _i in 0..n_blinks {
        stones = blink(&stones);
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::d11::main;

    const INPUT: &str = "125 17";

    #[test]
    fn test_example_1() {
        let result = main(INPUT, 25);
        assert_eq!(result, 55312);
    }
}

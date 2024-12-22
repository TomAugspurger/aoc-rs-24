use std::collections;

pub struct Lists {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
}

pub fn parse_input(input: &str) -> Lists {
    // Seems like just using Vecs and then
    // sorting is slightly faster on the benchmark?
    // let mut left = collections::BinaryHeap::new();
    // let mut right = collections::BinaryHeap::new();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();
        // iter.count()
        left.push(
            iter.next()
                .expect("Invalid line")
                .parse()
                .expect("Failed to parse number"),
        );
        right.push(iter.next().unwrap().parse().unwrap());

        assert!(iter.next().is_none());
    }

    left.sort();
    right.sort();

    // Lists { left: left.into_sorted_vec(), right: right.into_sorted_vec() }
    Lists { left, right }
}

pub fn find_distance(lists: &Lists) -> i64 {
    // there's a nightly into_iter_sorted()
    // needs to consume the Heap though...

    lists
        .left
        .iter()
        .zip(lists.right.iter())
        .map(|(a, b)| (*a as i64 - *b as i64).abs())
        .sum()
}

pub fn similarity_score(lists: &Lists) -> i64 {
    let mut counts = collections::HashMap::new();

    for num in lists.right.iter() {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut score: i64 = 0;

    for x in lists.left.iter() {
        score += (counts.get(x).unwrap_or(&0) * x) as i64;
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::d01::{find_distance, parse_input, similarity_score};

    #[test]
    fn test_part_1_example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let data = parse_input(input);
        let result = find_distance(&data);
        assert_eq!(result, 11);

        let result2 = similarity_score(&data);
        assert_eq!(result2, 31);
    }
}

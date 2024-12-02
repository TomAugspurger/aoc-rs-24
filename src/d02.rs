pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse().expect("Failed to parse digit from record."))
                .collect()
        })
        .collect()
}

/*
Damper logic:

This is "recoverable" if removing a level results in a safe report. How can we check that?

1. Naive: check all subslices. Nope.
2. Better: find the location of problems. Remove (around there) and check again.
3. Best: math?

Naively, you can try all the [n-1] slices. Nope.

You can also check *where* the issues are, remove around there, and try again. Better.
*/

pub fn is_safe_damped(record: &[u32], damped: bool) -> bool {
    let mut previous_sign = 0;

    let mut ok = true;

    for (i, window) in record.windows(2).enumerate() {
        if !(1..4).contains(&window[0].abs_diff(window[1])) {
            ok = false
        }
        let sign = (window[0] as i64 - window[1] as i64).signum();

        if i == 0 {
            previous_sign = sign
        } else if sign != previous_sign {
            ok = false
        }

        if !ok && !damped {
            break;
        } else if !ok {
            // We can try a few things:
            // 1. remove the previous
            // 2. remove the current
            // 3. remove the next
            // println!("i={i}. {}", i.min(1));
            let record2 = &[&record[..i.max(1) - 1], &record[i..]].concat();
            let mut r = is_safe_damped(record2, false);

            if !r {
                let record3 = &[&record[..i], &record[i + 1..]].concat();
                r = is_safe_damped(record3, false);
                // eprintln!("**Unsafe record @ {i}={record:?} -> {record2:?} -> {record3:?} -> {r2}**");
            }

            if !r {
                let record3 = &[&record[..i + 1], &record[i + 2..]].concat();
                r = is_safe_damped(record3, false);
                // eprintln!("**Unsafe record @ {i}={record:?} -> {record2:?} -> {record3:?} -> {r}**");
            }

            return r;
        }
    }

    ok
}

pub fn count_safe(report: &[Vec<u32>], damped: bool) -> usize {
    report
        .iter()
        .map(|x| is_safe_damped(x, damped))
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::d02::{is_safe_damped, parse_input};

    #[test]
    fn test_part_1_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let records = parse_input(input);
        let result: Vec<_> = records
            .iter()
            .map(|line| is_safe_damped(line, false))
            .collect();
        let expected = vec![true, false, false, false, false, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example() {
        let records = vec![6, 8, 11, 12, 14, 16, 18];
        let result = is_safe_damped(&records, false);
        assert!(result);
    }

    #[test]
    fn test_problem_edges() {
        let records = vec![0, 10, 11, 12, 13, 14];
        let result = is_safe_damped(&records, true);
        assert!(result);

        let result = is_safe_damped(&records, false);
        assert!(!result);

        let records = vec![1, 2, 3, 4, 10];
        let result = is_safe_damped(&records, true);
        assert!(result);

        let result = is_safe_damped(&records, false);
        assert!(!result);

        let records = vec![1, 2, 3, 100, 4, 5];
        let result = is_safe_damped(&records, true);
        assert!(result);

        let records = vec![1, 2, 3, 1, 4, 5];
        let result = is_safe_damped(&records, true);
        assert!(result);

        let records = vec![10, 9, 11, 12, 13, 14];
        let result = is_safe_damped(&records, true);
        assert!(result);
    }

    #[test]
    fn test_problem_initial() {
        let records = vec![7, 4, 7, 8, 9, 10];
        let result = is_safe_damped(&records, true);
        assert!(result);
    }

    #[test]
    fn test_part_2_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let records = parse_input(input);
        let expected = vec![true, false, false, true, true, true];
        let result: Vec<_> = records
            .iter()
            .map(|line| is_safe_damped(line, true))
            .collect();
        assert_eq!(result, expected);
    }
}

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

fn is_safe(record: &[u32]) -> bool {
    let mut gen = record.iter();
    let mut previous = gen.next().expect("Empty record");
    let mut is_safe = true;
    let mut difference: i64 = 0;

    for current in gen {
        let change = *current as i64 - *previous as i64;

        if difference != 0 && change.signum() != difference.signum() {
            is_safe = false
        }

        if change.abs() < 1 || change.abs() > 3 {
            is_safe = false
        }

        if !is_safe {
            break
        }

        previous = current;
        difference = change;
    }

    is_safe

}


fn is_safe_damper(record: &[u32]) -> bool {
    let mut gen = record.iter();
    let mut previous = gen.next().expect("Empty record");
    let mut is_safe = true;
    let mut difference: i64 = 0;
    let mut used_damper = false;

    for current in gen {
        let change = *current as i64 - *previous as i64;

        if difference != 0 && change.signum() != difference.signum() {
            is_safe = false
        }

        if change.abs() < 1 || change.abs() > 3 {
            is_safe = false
        }

        if !is_safe && used_damper {
            break
        }
        else if !is_safe {
            // We can ignore either the previous or current value:
            // Or can you also maybe eliminate the previous?
            // 1 5 4 3 2 1  -- eliminate the 1
            // 1 5 2 3 4 5  -- eliminate the 5
            




            used_damper = true
        }

        previous = current;
        difference = change;
    }

    is_safe

}




pub fn count_safe(report: &[Vec<u32>]) -> usize {
    report.iter().map(|x| is_safe(x)).filter(|b| *b).count()
} 

#[cfg(test)]
mod tests {
    use crate::d02::{is_safe, parse_input};


    #[test]
    fn test_part_1_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let records = parse_input(input);
        let result: Vec<_> = records.iter().map(|line| is_safe(line)).collect();
        let expected = vec![true, false, false, false, false, true];
        assert_eq!(result, expected)
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
        let result: Vec<_> = records.iter().map(|line| is_safe(line)).collect();
        let expected = vec![true, false, false, false, false, true];
        assert_eq!(result, expected)
    }


}

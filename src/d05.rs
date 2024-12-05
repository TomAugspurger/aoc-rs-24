/*

Basic strategy: I see there's a rule for each pair in the example.

If this is generally true, then we should be able to just to pairwise
checks that the constraint is satisfied at each step.

This wouldn't work if we needed to infer that some pair is valid,
like rules [A|B, B|C] and an ordering A,C. Search for "toplogoical sorting"
if that becomes needed.
*/
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Rule {
    pub left: u32,
    pub right: u32,
}

pub fn is_valid(constraints: &[Rule], ordering: &[u32]) -> bool {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule in constraints {
        map.entry(rule.left).or_default().insert(rule.right);
    }
    for window in ordering.windows(2) {
        // check if we're violating a rule.
        let maybe_rules = map.get(&window[1]);

        if let Some(x) = maybe_rules {
            // there is a rule saying that the RHS must be less than some values.
            if x.contains(&window[0]) {
                // There's a rule saying that our RHS must come before our LHS.
                // This ordering is invalid.
                // eprintln!("Invalid! {:?}", ordering);
                return false;
            }
        }
    }
    true
}

pub fn build_mapping(rules: &[Rule]) -> HashMap<u32, HashSet<u32>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule in rules {
        map.entry(rule.left).or_default().insert(rule.right);
    }
    map
}

#[derive(Clone, Eq, PartialEq)]
pub struct Rules<'a> {
    pub value: u32,
    pub rules: &'a HashMap<u32, HashSet<u32>>,
}

impl Ord for Rules<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        /*
        Cases:

        1. There's a rule for self.value *and* other is in it. Use that.
        2. There's a rule for self.value *and* other is not it. Check other.cmp(self)?
        3. There's no rule for self.value. Use other.cmp(self). If that's None,

        Do we us "Equal" for "no opinion"?
        */
        let maybe_rules = self.rules.get(&self.value);

        if let Some(rules) = maybe_rules {
            if rules.contains(&other.value) {
                // There's a rule indicating that we should come before other
                return std::cmp::Ordering::Less;
            }
        }

        let maybe_other_rules = self.rules.get(&other.value);

        if let Some(other_rules) = maybe_other_rules {
            if other_rules.contains(&self.value) {
                // there's a rule saying that other must come before self
                return std::cmp::Ordering::Greater;
            }
        }

        // There aren't any rules relating us. Let's pick... equal?
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Rules<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn fix_ordering(constraints: &[Rule], ordering: &[u32]) -> Vec<u32> {
    let rules = build_mapping(constraints);
    let x : Vec<_> = ordering.iter().map(|v| { Rules { value: *v, rules: &rules}}).collect();
    let heap = std::collections::BinaryHeap::from(x);
    let result: Vec<u32> = heap.into_sorted_vec().iter().map(|x| x.value).collect();
    result
}

pub fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let mut lines = input.lines();
    let mut constraints = Vec::new();
    let mut orderings = Vec::new();

    loop {
        let line = lines.next().expect("Unexpected EOF");
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once("|").expect("Invalid rule");

        constraints.push(Rule {
            left: left.parse().expect("Invalid left"),
            right: right.parse().expect("Invalid right"),
        })
    }

    for line in lines {
        orderings.push(
            line.split(",")
                .map(|x| x.parse().expect("Invalid ordering"))
                .collect(),
        );
    }

    (constraints, orderings)
}

fn midpoint(ordering: &[u32]) -> u32 {
    ordering[ordering.len() / 2]
}

pub fn sum_valid_middle(constraints: &[Rule], orderings: &[Vec<u32>]) -> u32 {
    orderings
        .iter()
        .filter(|ordering| is_valid(constraints, ordering))
        .map(|arg0: &std::vec::Vec<u32>| midpoint(arg0))
        .sum()
}

pub fn check(input: &str, fix_only: &bool) -> u32 {
    let (constraints, orderings) = parse_input(input);
    if !fix_only {
        sum_valid_middle(&constraints, &orderings)
    } else {
        let fixed: Vec<_> = orderings
            .iter()
            .filter(|ordering| !is_valid(&constraints, ordering))
            .map(|ordering| fix_ordering(&constraints, ordering))
            .collect();
        sum_valid_middle(&constraints, &fixed)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::d05::{build_mapping, check, fix_ordering, is_valid, Rule, Rules};

    use super::{parse_input, sum_valid_middle};

    #[test]
    fn test_part_1_example() {
        let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (constraints, orderings) = parse_input(input);

        let rules = build_mapping(&constraints);

        assert!(Rules { value: 47, rules: &rules} < Rules { value: 53, rules: &rules });
        assert!(Rules { value: 53, rules: &rules} > Rules { value: 47, rules: &rules });
        assert_eq!(Rules { value: 0, rules: &rules}.cmp(&Rules { value: 1, rules: &rules }), std::cmp::Ordering::Equal);

        assert!(is_valid(&constraints, &orderings[0]));
        assert!(!is_valid(&constraints, &orderings[3]));
        assert_eq!(sum_valid_middle(&constraints, &orderings), 143);

        // // 75,97,47,61,53 -> 97,75,47,61,53
        assert_eq!(
            fix_ordering(&constraints, &orderings[3]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(fix_ordering(&constraints, &orderings[4]), vec![61, 29, 13]);

        // // 97,13,75,29,47 ->
        // // 97,75,47,29,13
        assert_eq!(
            fix_ordering(&constraints, &orderings[5]),
            vec![97, 75, 47, 29, 13],
        );
        assert_eq!(check(input, &true), 123);
    }
}

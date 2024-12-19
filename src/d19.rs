// #[derive(Debug, PartialEq, Eq)]
// pub enum Color {
//     W,
//     U,
//     B,
//     R,
//     G,
// }

use std::collections::HashMap;

pub fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let stock: Vec<_> = input.lines().next().unwrap().split(", ").collect();

    let designs: Vec<_> = input.lines().skip(2).collect();
    (stock, designs)
}

pub fn is_possible<'a> (design: &'a str, options: & Vec<&'a str>, cache: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(&result) = cache.get(design) {
        return result;
    }

    if design.is_empty() {
        cache.insert(design, true);
        return true;
    }

    for towel in options.iter() {
        if design.starts_with(towel) && is_possible(&design[towel.len()..], options, cache) {
            cache.insert(design, true);
            return true;
        }
    }
    cache.insert(design, false);
    false
}

pub fn main(input: &str) -> usize {
    let (stock, designs) = parse_input(input);
    let cache = &mut HashMap::new();

    designs.iter().filter(|d| is_possible(d, &stock, cache)).count()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    const INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_example_1() {
        let (stock, designs) = super::parse_input(INPUT);
        assert_eq!(designs.len(), 8);
        assert_eq!(designs[0..2], vec!["brwrr", "bggr"]);

        assert!(super::is_possible("brwrr", &stock, &mut HashMap::new()));
        assert_eq!(super::main(INPUT), 6);
    }

    // #[test]
    // fn test_backtrack() {
    //     // Have a design like "brwrr"
    //     // and we match "too much" initially on "brw"
    //     // but we *need* a "wrr"
    //     let stock = HashMap::from_iter(vec![(1, vec![]), (2, vec!["br"]), (3, vec!["brw", "wrr"])]);
    //     assert!(super::is_possible("brwrr", &stock))
    // }
}

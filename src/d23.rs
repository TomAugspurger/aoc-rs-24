use std::collections::HashMap;

use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("-");
            let a = parts.next().unwrap().to_string();
            let b = parts.next().unwrap().to_string();
            (a, b)
        })
        .collect()
}

/*
Connect the pairs of computers into one or more networks where each
computer is connected to every other..
*/
pub fn connect_nodes(pairs: Vec<(String, String)>) -> Vec<Vec<String>> {
    // let mut components: Vec<Vec<String>> = Vec::new();
    let all_nodes: Vec<_> = pairs
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .unique()
        .collect();
    let mut connected: HashMap<&str, Vec<String>> = HashMap::new();
    for (a, b) in pairs.iter() {
        connected.entry(a).or_default().push(b.clone());
        connected.entry(b).or_default().push(a.clone());
    }

    let mut result: Vec<Vec<String>> = Vec::new();

    // Find the triplets where each node is connected to the other two.
    for node in all_nodes.iter() {
        if let Some(neighbors) = connected.get(node.as_str()) {
            for neighbor in neighbors.iter() {
                if let Some(neighbors2) = connected.get(neighbor.as_str()) {
                    for neighbor2 in neighbors2.iter() {
                        if neighbor2 != *node && neighbors.contains(neighbor2) {
                            let mut triplet =
                                vec![node.to_string(), neighbor.clone(), neighbor2.clone()];
                            triplet.sort();
                            result.push(triplet);
                        }
                    }
                }
            }
        }
    }

    result.sort();
    result.dedup();
    result
}

pub fn main(input: &str) -> u64 {
    let pairs = parse_input(input);
    // eprintln!("{pairs:?}");
    let networks = connect_nodes(pairs);
    // eprintln!("{networks:#?}");
    let t_networks: Vec<_> = networks
        .iter()
        .filter(|network| network.iter().filter(|node| node.starts_with("t")).count() > 0)
        .collect();

    t_networks.len() as u64
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_example_1() {
        let result = main(INPUT);
        assert_eq!(result, 7);
    }
}

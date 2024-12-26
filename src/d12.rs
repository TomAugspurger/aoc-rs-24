/*
We're given an input with various Plants. We need to
form those plants into Regions, where a Region is a
group of connected Plants of the same type.

Then, for each Region, we calculate the cost (of fencing)
as each region's Area * Perimeter.
*/
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Field {
    pub plots: Vec<Plot>,
    pub n_rows: usize,
    pub n_cols: usize,
    // pub regions: HashMap<char, Vec<(usize, usize)>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Plot {
    pub label: char,
    pub row: usize,
    pub col: usize,
}

pub fn parse_input(input: &str) -> Field {
    let mut plots = Vec::new();
    let n_rows = input.matches("\n").count() + 1;
    let mut n_cols = 0;
    for (row, line) in input.lines().enumerate() {
        n_cols = line.len();
        for (col, label) in line.chars().enumerate() {
            plots.push(Plot {
                label,
                row,
                col,
                // region: None,
            });
        }
    }
    Field {
        plots,
        n_rows,
        n_cols,
        // regions: HashMap::new(),
    }
}

impl Field {
    /*
    The neighbors of a plot that have the same label.
     */

    pub fn get(&self, row: usize, col: usize) -> Option<&Plot> {
        self.plots.get(row * self.n_cols + col)
    }

    pub fn neighbors(&self, plot: &Plot) -> Vec<Option<&Plot>> {
        let mut neighbors = Vec::new();

        if plot.row > 0 {
            neighbors.push(self.get(plot.row - 1, plot.col));
        }

        if plot.row < self.n_rows - 1 {
            neighbors.push(self.get(plot.row + 1, plot.col));
        }

        if plot.col > 0 {
            neighbors.push(self.get(plot.row, plot.col - 1));
        }
        if plot.col < self.n_cols - 1 {
            neighbors.push(self.get(plot.row, plot.col + 1));
        }

        neighbors
    }

    pub fn matching_neighbors(&self, plot: &Plot) -> Vec<&Plot> {
        self.neighbors(plot)
            .iter()
            .filter_map(|p| {
                if let Some(p) = p {
                    if p.label == plot.label {
                        return Some(*p);
                    }
                }
                None
            })
            .collect()
    }
}

pub fn main(input: &str) -> u64 {
    let field = parse_input(input);
    let mut regions: Vec<HashSet<&Plot>> = Vec::new();
    let mut all_points: HashSet<&Plot> = HashSet::new();

    // https://advent-of-code.xavd.id/writeups/2024/day/12/ has a nice write up.
    for plot in field.plots.iter() {
        if all_points.contains(&plot) {
            continue;
        }

        // We're the root of a new region. Congrats.
        let mut region = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(plot);

        // Depth-first search over ...
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if region.contains(&current) {
                continue;
            }

            region.insert(current);

            for neighbor in field.matching_neighbors(current) {
                queue.push_back(neighbor);
            }
        }

        all_points.extend(&region);
        regions.push(region);
    }

    let mut result = 0;

    for region in regions.iter() {
        let mut perimeter = 0;
        let area = region.len();
        // let mut area = 0;
        for plot in region.iter() {
            perimeter += 4 - field.matching_neighbors(&plot).len();
        }
        result += perimeter * area;
    }

    result as u64
}
#[cfg(test)]
mod tests {
    use crate::d12::main;

    const INPUT1: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const INPUT2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const INPUT3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_example_1() {
        assert_eq!(main(INPUT1), 140);
        assert_eq!(main(INPUT2), 772);
        assert_eq!(main(INPUT3), 1930);
    }
}

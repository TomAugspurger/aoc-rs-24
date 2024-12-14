// use std::{
//     collections::{HashMap, HashSet},
//     vec,
// };

// use itertools::Itertools;

// // use crate::utils::Grid;

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Field {
    pub plots: Vec<Plot>,
    pub n_rows: usize,
    pub n_cols: usize,
    pub regions: HashMap<char, Vec<(usize, usize)>>,
}

#[derive(Debug)]
pub struct Plot {
    pub label: char,
    pub row: usize,
    pub col: usize,
    // the root node?
    // pub region: Option<(usize, usize)>,
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
        regions: HashMap::new(),
    }
}

impl Field {
    pub fn get(&self, row: usize, col: usize) -> Option<&Plot> {
        let index = row * self.n_cols + col;
        self.plots.get(index)
    }

    pub fn matching_neighbors(&self, plot: &Plot) -> Vec<&Plot> {
        let neighbors = [
            (plot.row.checked_sub(1), Some(plot.col)),
            (
                if plot.row < self.n_rows - 1 {
                    plot.row.checked_add(1)
                } else {
                    None
                },
                Some(plot.col),
            ),
            (Some(plot.row), plot.col.checked_sub(1)),
            (
                Some(plot.row),
                if plot.col < self.n_cols - 1 {
                    plot.col.checked_add(1)
                } else {
                    None
                },
            ),
        ];

        let matches: Vec<_> = neighbors
            .iter()
            .filter_map(|(r, c)| match (r, c) {
                (Some(r), Some(c)) => {
                    let neighbor = self.get(*r, *c).unwrap();
                    if neighbor.label == plot.label {
                        Some(neighbor)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        matches
    }
}

pub fn add_plot(
    plot: &Plot,
    field: &Field,
    regions: &mut Vec<HashSet<(usize, usize)>>,
    visited: &mut HashSet<(usize, usize)>,
) {
    if regions.is_empty() {
        eprintln!("First entry for {} @ ({}, {})", plot.label, plot.row, plot.col);
        let mut new = HashSet::new();
        new.insert((plot.row, plot.col));
        regions.push(new);
    } else {
        visited.insert((plot.row, plot.col));
        let matching_neighbors = field.matching_neighbors(plot);
        let mut missing = true;

        if matching_neighbors.is_empty() {
            eprintln!("Orphan entry for {}", plot.label);
            let mut new = HashSet::new();
            new.insert((plot.row, plot.col));
            regions.push(new);
        } else {
            for neighbor in matching_neighbors.iter() {
                for region in regions.iter_mut() {
                    if region.contains(&(neighbor.row, neighbor.col)) {
                        // our neighbor is part of a region. Join it
                        eprintln!("{plot:?} joins {neighbor:?}");
                        region.insert((plot.row, plot.col));
                        missing = false;
                        break;
                    }
                }
                // Our neighbor doesn't have a thing. Search its neighbors.
                if missing && !visited.contains(&(neighbor.row, neighbor.col)) {
                    eprintln!("{plot:?} searches {neighbor:?} neighbors={matching_neighbors:?} regions={regions:?}");
                    add_plot(neighbor, field, regions, visited);
                }
            }
            // Final check: did we update any of our neighbors?
            for region in regions.iter_mut() {
                for neighbor in matching_neighbors.iter() {
                    if region.contains(&(neighbor.row, neighbor.col)) {
                        // our neighbor was added, add ourself too
                        region.insert((plot.row, plot.col));
                        missing = false;
                        break
                    }
                }
            }

            if missing {
                eprintln!("new group for {}", plot.label);
                let mut new = HashSet::new();
                new.insert((plot.row, plot.col));
                regions.push(new);
            }
       }
    }
}

pub fn form_regions(field: &Field) -> HashMap<char, Vec<HashSet<(usize, usize)>>> {
    let mut regions: HashMap<char, Vec<HashSet<(usize, usize)>>> = HashMap::new();

    for plot in field.plots.iter() {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let plant_regions = regions.entry(plot.label).or_default();
        add_plot(plot, field, plant_regions, &mut visited);
    }

    // eprint!("{regions:?}");
    regions
}

pub fn main(input: &str) -> u64 {
    let field = parse_input(input);
    let region_maps = form_regions(&field);

    let mut result = 0;
    for (char, regions) in region_maps.iter() {
        for region in regions.iter() {
            let area = region.len();
            let mut perimeter = 0;
            for plot in region {
                perimeter += 4 - field
                    .matching_neighbors(field.get(plot.0, plot.1).unwrap())
                    .len();
            }
            let region_cost = area * perimeter;
            eprintln!("char={char} cost={region_cost:?} region={region:?}");
            result += area * perimeter;
        }
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

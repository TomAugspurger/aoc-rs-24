use std::collections::{HashMap, HashSet};

use itertools::Itertools;

/*

Decision: how to represent the grid of antenna and antinodes?

1. Vec<Vec<char>>. Matches the input diagrams
2. HashMap<char, Vec<(usize, usize)>. Simpler to work with

Choosing the hashmap I think.

*/
type Locations = HashMap<char, Vec<(usize, usize)>>;

pub struct Grid {
    pub antennae: Locations,
    pub n_rows: usize,
    pub n_cols: usize,
}

pub fn parse_input(input: &str) -> Grid {
    let mut antennae: Locations = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                antennae.entry(char).or_default().push((i, j));
            }
        }
    }

    Grid {
        antennae,
        n_rows: input.lines().count(),
        n_cols: input.lines().next().unwrap().len(),
    }
}

pub fn find_antinodes(grid: &Grid) -> HashSet<(usize, usize)> {
    let mut locations = HashSet::new();
    for (_char, positions) in grid.antennae.iter() {
        for combination in positions.iter().combinations(2) {
            let (a, b) = (combination[0], combination[1]);
            let delta = ((a.0 as i32 - b.0 as i32), (a.1 as i32 - b.1 as i32));

            let abs_delta: (i32, i32) = (delta.0.abs(), delta.1.abs());

            if delta.0 <= 0 && delta.1 <= 0 {
                // Move NW from 'a' and SE from 'b'
                let c_a = (a.0 as i32 - abs_delta.0, a.1 as i32 - abs_delta.1);
                let c_b = (b.0 as i32 + abs_delta.0, b.1 as i32 + abs_delta.1);

                if c_a.0 >= 0 && c_a.1 >= 0 {
                    // eprintln!("Found NW antinode! char={_char:?} a={a:?} b={b:?} p={c_a:?} delta={delta:?} abs_delta={abs_delta:?}");
                    locations.insert((c_a.0 as usize, c_a.1 as usize));
                }

                if (c_b.0 as usize) < grid.n_rows && (c_b.1 as usize) < grid.n_cols {
                    // eprintln!("Found SE antinode! char={_char:?} a={a:?} b={b:?} p={c_b:?} delta={delta:?} abs_delta={abs_delta:?}");
                    locations.insert((c_b.0 as usize, c_b.1 as usize));
                }
            } else if delta.0 <= 0 && delta.1 > 0 {
                // Move NE from 'a' and SW from 'b'
                let c_a = (a.0 as i32 - abs_delta.0, a.1 as i32 + abs_delta.1);
                let c_b = (b.0 as i32 + abs_delta.0, b.1 as i32 - abs_delta.1);

                if c_a.0 >= 0 && (c_a.1 as usize) < grid.n_cols {
                    // eprintln!("Found NE antinode! char={_char:?} a={a:?} b={b:?} p={c_a:?} delta={delta:?} abs_delta={abs_delta:?}");
                    locations.insert((c_a.0 as usize, c_a.1 as usize));
                }

                if (c_b.0 as usize) < grid.n_rows && c_b.1 >= 0 {
                    // eprintln!("Found SW antinode! char={_char:?} a={a:?} b={b:?} p={c_b:?} delta={delta:?} abs_delta={abs_delta:?}");
                    locations.insert((c_b.0 as usize, c_b.1 as usize));
                }
            } else if delta.0 > 0 && delta.1 <= 0 {
                // our parse order means we don't hit this from the CLI.
                todo!()
            } else {
                // delta.0 > 0 && delta.1 > 0
                // our parse order means we don't hit this from the CLI.
                todo!()
            }
        }
    }

    locations
}


pub fn format_antinodes(grid: &Grid, antinodes: &HashSet<(usize, usize)>) -> String{
    let mut buf = String::new();

    for row in 0..grid.n_rows {
        for col in 0..grid.n_cols {
            if antinodes.contains(&(row, col)) {
                buf.push('#');
            }

            else {
                buf.push('.');
            }
        }

        buf.push('\n');
    }

    buf
}


pub fn count_antinodes(input: &str) -> usize {
    let grid = parse_input(input);
    let antinodes = find_antinodes(&grid);
    // let formatted = format_antinodes(&grid, &antinodes);
    // eprintln!("{formatted}");
    // eprintln!("{antinodes:?}");
    // assert_eq!(antinodes.len(), 394);
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use crate::d08::find_antinodes;

    use super::parse_input;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn test_parse_input() {
        let result = parse_input(INPUT);

        assert_eq!(
            result.antennae.get(&'A'),
            Some(&vec![(5, 6), (8, 8), (9, 9)])
        );
        assert_eq!(result.n_rows, 12);
        assert_eq!(result.n_cols, 12);
    }

    #[test]
    fn test_example_1() {
        let data = parse_input(INPUT);
        let result = find_antinodes(&data);
        assert_eq!(result.len(), 14);
    }
}

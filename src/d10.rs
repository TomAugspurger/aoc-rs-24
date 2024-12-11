// use itertools::Itertools;

use itertools::Itertools;

pub struct Grid {
    pub values: Vec<Vec<u8>>,
}

impl Grid {
    pub fn n_rows(&self) -> usize {
        self.values.len()
    }

    pub fn n_cols(&self) -> usize {
        self.values[0].len()
    }

    pub fn up(&self, row: usize, col: usize) -> Option<&u8> {
        self.iloc(row - 1, col)
    }

    pub fn down(&self, row: usize, col: usize) -> Option<&u8> {
        self.iloc(row + 1, col)
    }

    pub fn left(&self, row: usize, col: usize) -> Option<&u8> {
        self.iloc(row, col - 1)
    }
    pub fn right(&self, row: usize, col: usize) -> Option<&u8> {
        self.iloc(row, col + 1)
    }

    pub fn iloc(&self, row: usize, col: usize) -> Option<&u8> {
        self.values.get(row)?.get(col)
    }
    //     eprintln!("row={row:} col={col}");
    //     self.values.get(0).or_else(|r| r.get)
    //     if (row >= self.n_rows()) || (col == 0 || col == self.n_cols() - 1) {
    //         None
    //     } else {
    //         Some(self.values[row][col])
    //     }
    // }

    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let choices = [
            (row.checked_sub(1), Some(col)),
            (row.checked_add(1), Some(col)),
            (Some(row), col.checked_sub(1)),
            (Some(row), col.checked_add(1)),
        ];
        choices
            .iter()
            .filter(|(r, c)| {
                r.is_some()
                    && c.is_some()
                    && r.unwrap() < self.n_rows()
                    && c.unwrap() < self.n_cols()
            })
            .map(|a| (a.0.unwrap(), a.1.unwrap()))
            .collect()
    }
}

pub fn parse_input(input: &str) -> Grid {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as u8);
        }
        result.push(row);
    }
    Grid { values: result }
}

pub fn find_trailheads_candidates(grid: &Grid) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for (i, row) in grid.values.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            // must trailheads be on the edge?
            // if i == 0 || i == m || j == 0 || j == n && *val == 0 {
            if *val == 0 {
                result.push((i, j));
            }
        }
    }

    result
}

pub fn find_trails(grid: &Grid, row: &usize, col: &usize, value: &u8) -> Vec<(usize, usize)> {
    if *value == 9 {
        vec![(*row, *col)]
    } else {
        let neighbors = grid.neighbors(*row, *col);
        // eprintln!("row={row} col={col} value={value} neighbors={neighbors:?}");
        neighbors
            .iter()
            .filter(|(r, c)| grid.iloc(*r, *c).unwrap() == &(value + 1))
            .flat_map(|(r, c)| find_trails(grid, r, c, &(value + 1)))
            .unique()
            .collect()
    }
}

pub fn find_trails2(
    grid: &Grid,
    row: &usize,
    col: &usize,
    value: &u8,
    history: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let new_history = [history, &[(*row, *col)]].concat();
    if *value == 9 {
        new_history
    } else {
        let neighbors = grid.neighbors(*row, *col);
        eprintln!("row={row} col={col} value={value} neighbors={neighbors:?} new_history=:{new_history:?}");
        neighbors
            .iter()
            .filter(|(r, c)| grid.iloc(*r, *c).unwrap() == &(value + 1))
            .flat_map(|(r, c)| find_trails2(grid, r, c, &(value + 1), &new_history))
            .sorted()
            .inspect(|&x| {
                eprintln!("{x:?} {:?}", grid.iloc(x.0, x.1));
            })
            .unique()
            .collect()
    }
}

// pub fn score_trailhead_candidate(grid: &Grid, row: usize, col: usize) -> u32 {
//     /*
//     We start at some row, col with a value of '0'.

//     We search all our neighbors for a value of '1'. Those neighbors in turn
//     search *their* neighbors for a value of '2'...

//     Eventually, we reach '9' for some of the paths, at which point we conclude we have a trail.

//     We count up the number of paths that reach '9'.
//     */
//     assert_eq!(grid.iloc(row, col), Some(0));
//    }

//     todo!()
// }

pub fn sum_trailhead_scores(grid: &Grid, as_ratings: bool) -> u64 {
    let trailheads = find_trailheads_candidates(grid);
    // let score = trailheads.iter().map(|(r, c)| count_trails(grid, r, c, &0)).sum();
    // eprintln!("Starts: {:?}", trailheads);
    let x = if as_ratings {
        trailheads
            .iter()
            .map(|(r, c)| find_trails2(grid, r, c, &0, &[(*r, *c)]).len())
            .sum::<usize>()
    } else {
        trailheads
            .iter()
            .map(|(r, c)| find_trails(grid, r, c, &0).len())
            .sum()
    } as u64;

    x
}

pub fn main(input: &str, as_ratings: bool) -> u64 {
    let grid = parse_input(input);
    sum_trailhead_scores(&grid, as_ratings)
}

#[cfg(test)]
mod tests {
    use super::main;

    const INPUT_1: &str = "\
0123
1234
8765
9876";

    const INPUT_2: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_example_1() {
        let result = main(INPUT_1, false);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_2() {
        let result = main(INPUT_2, false);
        assert_eq!(result, 36);
    }

    // #[test]
    // fn test_example_2_part_2() {
    //     let result = main(INPUT_2, true);
    //     assert_eq!(result, 81);
    // }
}

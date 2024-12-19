use std::collections::{HashMap, VecDeque};

pub struct Grid {
    incoming: Vec<(usize, usize)>,
    n_rows: usize,
    n_cols: usize,
    is_safe: Vec<bool>,
}

impl Grid {
    pub fn get(&self, row: usize, col: usize) -> Option<&bool> {
        let index = row * self.n_cols + col;
        self.is_safe.get(index)
    }

    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        let index = row * self.n_cols + col;
        self.is_safe[index] = value
    }

    pub fn safe_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let nodes = [
            (row.checked_sub(1), Some(col)),
            (row.checked_add(1), Some(col)),
            (Some(row), col.checked_sub(1)),
            (Some(row), col.checked_add(1)),
        ];
        nodes
            .iter()
            .filter_map(|(r, c)| {
                if r.is_some() && c.is_some() {
                    let v = self.get(r.unwrap(), c.unwrap());
                    if v.is_some() && *v.unwrap() {
                        Some((r.unwrap(), c.unwrap()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Grid {
    pub fn run(&mut self, n_steps: usize) -> Vec<(usize, usize)> {
        for i in 0..n_steps {
            let (col, row) = self.incoming[i];
            self.set(row, col, false);
        }

        // (0, 0) -> (n_rows, n_cols)
        let target = (self.n_rows - 1, self.n_cols - 1);
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        let mut visited = HashMap::new();
        // visited.insert((0, 0));

        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();
            let neighbors = self.safe_neighbors(row, col);
            for neighbor in neighbors.iter() {
                if *neighbor == target {
                    visited.insert(*neighbor, (row, col));
                    break
                }

                if !visited.contains_key(neighbor) {
                    visited.insert(*neighbor, (row, col));
                    queue.push_back(*neighbor);
                }

            }

        }

        // now go back from the target to the start.
        let mut path = Vec::new();
        let mut node = target;
        while node != (0, 0) {
            node = visited[&node];
            path.push(node);
        }

        path.reverse();
        path

    }
}

pub fn parse_input(input: &str, n_rows: usize, n_cols: usize) -> Grid {
    let incoming: Vec<_> = input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let size = n_cols * n_rows;

    Grid {
        incoming,
        n_rows,
        n_cols,
        is_safe: vec![true; size],
    }
}

pub fn main(input: &str, n_rows: usize, n_cols: usize, n_steps: usize) -> usize {
    let mut state = parse_input(input, n_rows, n_cols);
    let path = state.run(n_steps);
    path.len()
}

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_example_1() {
        let mut state = parse_input(INPUT, 7, 7);
        assert_eq!(state.incoming[0], (5, 4));
        let path = state.run(12);
        assert_eq!(path.len(), 22);
    }
}

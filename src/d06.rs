/*

https://adventofcode.com/2024/day/6

Basic idea:

1. Store the state as
    - 2-d array of bools (indicating points visited)
    - current position
    - current direction
    - location of obstacles
*/

use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Guard {
    Missing,
    Position(usize, usize),
}

pub struct State {
    pub visited: Vec<Vec<bool>>, // index with [row][col]
    pub position: Guard,
    pub direction: Direction,
    pub obstacles: Vec<(usize, usize)>,
}

pub fn parse_input(input: &str) -> State {
    let mut obstacles = Vec::new();
    let lines = input.lines();
    let mut m = 0;
    let mut n = 0;
    let mut direction = Direction::N;
    let mut position = (0, 0);

    for (i, line) in lines.enumerate() {
        n = line.len();
        m = i;
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => obstacles.push((i, j)),
                '^' => {
                    direction = Direction::N;
                    position = (i, j);
                }
                '>' => {
                    direction = Direction::E;
                    position = (i, j);
                }
                'v' => {
                    direction = Direction::S;
                    position = (i, j);
                }
                '<' => {
                    direction = Direction::W;
                    position = (i, j);
                }
                _ => {}
            }
        }
    }

    let mut visited = vec![vec![false; n]; m + 1];
    visited[position.0][position.1] = true;
    State {
        visited,
        position: Guard::Position(position.0, position.1),
        direction,
        obstacles,
    }
}

pub fn count_positions(input: &str) -> u32 {
    let mut state = parse_input(input);
    state.run();
    // eprintln!("{}", String::from_utf8_lossy(&state.format_visited()));
    state.count_visited()
}

fn update_position(
    position: &(usize, usize),
    direction: &Direction,
    obstacles: &[(usize, usize)],
) -> Guard {
    // When facing N/S, we'll keep our 'x' and move our 'y'.
    // When facing W/E, vice-versa.
    let (row, col) = position;
    match direction {
        // - When facing N, we want the *largest* 'row' s.t. row < current row
        // TODO: fix all the other filters.
        // TODO: Shift by 1 back in the direction we came from. You just hit the obstacle, not stand on it.
        Direction::N => {
            let candidates: Vec<_> = obstacles
                .iter()
                .filter(|(ob_row, ob_col)| ob_row < row && ob_col == col)
                .collect();
            // eprintln!("candidates: {candidates:?}");
            if candidates.is_empty() {
                Guard::Missing
            } else {
                let p = candidates.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap();
                assert!(obstacles.contains(p));
                Guard::Position(p.0 + 1, p.1)
            }
        }

        // - When facing S, we want the *smallest* 'row' s.t. row > current row
        Direction::S => {
            let candidates: Vec<_> = obstacles
                .iter()
                .filter(|(ob_row, ob_col)| ob_row > row && ob_col == col)
                .collect();
            // eprintln!("candidates: {candidates:?}");
            if candidates.is_empty() {
                Guard::Missing
            } else {
                let p = candidates.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
                assert!(obstacles.contains(p));
                Guard::Position(p.0 - 1, p.1)
            }
        }

        // - When facing W, we want the *largest* 'col' s.t. col < current col
        Direction::W => {
            let candidates: Vec<_> = obstacles
                .iter()
                .filter(|(ob_row, ob_col)| ob_row == row && ob_col < col)
                .collect();
            // eprintln!("candidates: {candidates:?}");
            if candidates.is_empty() {
                Guard::Missing
            } else {
                let p = candidates.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap();
                assert!(obstacles.contains(p));
                Guard::Position(p.0, p.1 + 1)
            }
        }

        // - When facing E, we want the *smallest* 'col' s.t. col > current col
        Direction::E => {
            let candidates: Vec<_> = obstacles
                .iter()
                .filter(|(ob_row, ob_col)| ob_row == row && ob_col > col)
                .collect();
            // eprintln!("candidates: {candidates:?}");
            if candidates.is_empty() {
                Guard::Missing
            } else {
                let p = candidates.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
                assert!(obstacles.contains(p));
                Guard::Position(p.0, p.1 - 1)
            }
        }
    }
}

impl Guard {
    pub fn position(&self) -> Option<(usize, usize)> {
        match &self {
            Guard::Missing => None,
            Guard::Position(x, y) => Some((*x, *y)),
        }
    }
}

impl State {
    pub fn n_rows(&self) -> usize {
        self.visited.len()
    }
    pub fn n_cols(&self) -> usize {
        self.visited[0].len()
    }

    pub fn format_visited(&self) -> Vec<u8> {
        let mut buf =
            Vec::with_capacity(self.n_rows() * self.n_cols() + self.n_rows() + self.n_cols());

        for row in self.visited.iter() {
            for col in row.iter() {
                if *col {
                    buf.push(b'X');
                } else {
                    buf.push(b'.');
                }
            }
            buf.push(b'\n');
        }

        // 0, 11, 22, 33
        // row * 10 + row
        for (row, col) in self.obstacles.iter() {
            let idx = row * self.n_cols() + col + row;
            buf[idx] = b'#'
        }

        buf
    }

    pub fn run(&mut self) {
        // Figure out the positions we visit, based on
        // 1. our current position
        // 2. the direction we're facing
        // 3. the next obstacle we hit

        // cleanup: generate the indices (based on direction & position) then set once

        let mut history = HashSet::new();

        loop {
            if history.contains(&(self.direction.clone(), self.position.clone())) {
                eprintln!("Cycle detected!");
                eprintln!(
                    "Direction: {:?}, Position: {:?}",
                    self.direction, self.position
                );
                panic!("Cycle detected!")
            }

            history.insert((self.direction.clone(), self.position.clone()));
            match self.position {
                Guard::Missing => break,
                Guard::Position(row, col) => {
                    let new_spot = update_position(&(row, col), &self.direction, &self.obstacles);
                    // eprintln!(
                    //     "Move {:?} from {:?} -> {:?}",
                    //     self.direction, self.position, new_spot
                    // );

                    match new_spot {
                        Guard::Missing => {
                            // 1. visited
                            // We've exited the space. Fill zeros from the previous position to the edge in direction.
                            match self.direction {
                                Direction::N => {
                                    // We're currently at (row, col)
                                    for j in 0..=row {
                                        self.visited[j][col] = true
                                    }
                                }

                                Direction::S => {
                                    // column is fixed, fill down
                                    for j in row..self.n_rows() {
                                        self.visited[j][col] = true
                                    }
                                }

                                Direction::W => {
                                    for i in 0..=col {
                                        self.visited[row][i] = true
                                    }
                                }

                                Direction::E => {
                                    for i in col..self.n_cols() {
                                        self.visited[row][i] = true
                                    }
                                }
                            }

                            // 2. position
                            self.position = new_spot;
                        }
                        Guard::Position(new_row, new_col) => {
                            // update our state
                            // 1. visited
                            match self.direction {
                                Direction::N => {
                                    for j in new_row..row {
                                        self.visited[j][col] = true
                                    }
                                    self.direction = Direction::E;
                                }
                                Direction::S => {
                                    for j in row..new_row + 1 {
                                        self.visited[j][col] = true
                                    }
                                    self.direction = Direction::W;
                                }
                                Direction::W => {
                                    for i in new_col..col {
                                        self.visited[row][i] = true
                                    }
                                    self.direction = Direction::N;
                                }
                                Direction::E => {
                                    for i in col..new_col + 1 {
                                        self.visited[row][i] = true
                                    }
                                    self.direction = Direction::S
                                }
                            }

                            // 2. position
                            self.position = new_spot;
                        }
                    }
                }
            }
        }
    }
    pub fn count_visited(&self) -> u32 {
        // eprintln!("Visited: {:#?}", self.visited);
        self.visited.iter().flatten().filter(|b| **b).count() as u32
    }
}

#[cfg(test)]
mod tests {
    use crate::d06::{count_positions, parse_input, Direction, Guard};

    use super::update_position;
    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    //     const FINAL: &str = "\
    // ....#.....
    // ....XXXXX#
    // ....X...X.
    // ..#.X...X.
    // ..XXXXX#X.
    // ..X.X.X.X.
    // .#XXXXXXX.
    // .XXXXXXX#.
    // #XXXXXXX..
    // ......#X..";

    #[test]
    fn test_parse_input() {
        let state = parse_input(INPUT);
        assert_eq!(state.direction, Direction::N);
        assert_eq!(state.position, Guard::Position(6, 4));
        assert_eq!(
            state.obstacles,
            vec![
                (0, 4),
                (1, 9),
                (3, 2),
                (4, 7),
                (6, 1),
                (7, 8),
                (8, 0),
                (9, 6)
            ]
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(count_positions(INPUT), 41);
    }

    #[test]
    fn test_update_position() {
        let result = update_position(&(0, 0), &Direction::E, &[(0, 2), (0, 3)]);
        let expected = Guard::Position(0, 1);
        assert_eq!(result, expected);

        let result = update_position(&(0, 10), &Direction::W, &[(0, 2), (0, 3)]);
        let expected = Guard::Position(0, 4);
        assert_eq!(result, expected);

        let result = update_position(&(0, 0), &Direction::S, &[(2, 0), (3, 0)]);
        let expected = Guard::Position(1, 0);
        assert_eq!(result, expected);

        let result = update_position(&(10, 0), &Direction::N, &[(2, 0), (3, 0)]);
        let expected = Guard::Position(4, 0);
        assert_eq!(result, expected);

        let result = update_position(&(0, 4), &Direction::E, &[(0, 2), (0, 3)]);
        let expected = Guard::Missing;
        assert_eq!(result, expected);

        let result = update_position(&(0, 1), &Direction::W, &[(0, 2), (0, 3)]);
        let expected = Guard::Missing;
        assert_eq!(result, expected);

        let result = update_position(&(4, 0), &Direction::S, &[(2, 0), (3, 0)]);
        let expected = Guard::Missing;
        assert_eq!(result, expected);

        let result = update_position(&(1, 0), &Direction::N, &[(2, 0), (3, 0)]);
        let expected = Guard::Missing;
        assert_eq!(result, expected);
    }
}

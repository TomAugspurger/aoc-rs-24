#[derive(PartialEq, Debug)]
pub enum Object {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Move {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
pub struct Map {
    pub grid: Vec<Object>,
    pub moves: Vec<Move>,
    pub n_rows: usize, // including walls
    pub n_cols: usize, // including walls
}

pub fn parse_input(input: &str) -> Map {
    let mut grid = Vec::new();
    let mut moves = Vec::new();
    let lines = input.lines();
    let mut n_rows = 0;
    let mut n_cols = 0;

    for (i, line) in lines.enumerate() {
        if line.starts_with('#') {
            n_rows = i;
            n_cols = line.len();
            for c in line.chars() {
                let value = match c {
                    '#' => Object::Wall,
                    'O' => Object::Box,
                    '.' => Object::Empty,
                    '@' => Object::Robot,
                    _ => panic!("Unexpected character"),
                };
                grid.push(value);
            }
        } else if line.is_empty() {
            continue;
        } else {
            for c in line.chars() {
                if c != '\n' {
                    let v = match c {
                        '^' => Move::U,
                        'v' => Move::D,
                        '<' => Move::L,
                        '>' => Move::R,
                        _ => panic!("Unexpected character in move set."),
                    };
                    moves.push(v);
                }
            }
        }
    }

    Map {
        grid,
        moves,
        n_rows: n_rows + 1,
        n_cols,
    }
}

impl Map {
    pub fn get(&self, row: usize, col: usize) -> Option<&Object> {
        let index = row * self.n_cols + col;
        self.grid.get(index)
    }
    pub fn set(&mut self, row: usize, col: usize, value: Object) {
        let index = row * self.n_cols + col;
        self.grid[index] = value
    }

    // pub fn get_up(&self, row: usize, col: usize) -> Option<&Object> {
    //     self.get(row.wrapping_sub(1), col)
    // }

    // pub fn get_down(&self, row: usize, col: usize) -> Option<&Object> {
    //     self.get(row.wrapping_add(1), col)
    // }

    // pub fn get_left(&self, row: usize, col: usize) -> Option<&Object> {
    //     self.get(row, col.wrapping_sub(1))
    // }
    // pub fn get_right(&self, row: usize, col: usize) -> Option<&Object> {
    //     self.get(row, col.wrapping_add(1))
    // }
    // pub fn get_direction(&self, direction: Move, row: usize, col: usize) -> Option<&Object> {
    //     match direction {
    //         Move::U => self.get_up(row, col),
    //         Move::D => self.get_down(row, col),
    //         Move::L => self.get_left(row, col),
    //         Move::R => self.get_right(row, col),
    //     }
    // }

    pub fn step(&mut self) {
        // find the position of the robot.
        let index = self.grid.iter().position(|r| *r == Object::Robot).unwrap();

        let (mut row, mut col) = (index / self.n_cols, index % self.n_cols);

        // Not sure how to avoid this clone. We *are* mutating self, but just self.grid; not `moves`.
        for direction in self.moves.clone().iter() {
            // where the robot will move to.
            let mut pending_robot = None;
            // where the new box will be (ignore chains, just "move" from the end to the new spot)
            let mut pending_box = None;
            // where the new empty spot will be from where the robot was.
            // Alternatively, we could just undo the move in direction.

            let (mut pending_row, mut pending_col) = (row, col);
            // Some weirdness around the loop and what we're searching for here.
            // - (row, col) the *actual* position of the robot.
            // - (pending_row, pending_col) is where the robot will end up *if* we
            //   move it.
            // - ()
            // eprintln!("Move {direction:?} from ({row}, {col})");

            loop {
                (pending_row, pending_col) = match direction {
                    Move::U => (pending_row - 1, pending_col),
                    Move::D => (pending_row + 1, pending_col),
                    Move::L => (pending_row, pending_col - 1),
                    Move::R => (pending_row, pending_col + 1),
                };
                let new_object = self.get(pending_row, pending_col).unwrap();
                // eprintln!("Hit {new_object:?} @ ({pending_col}, {pending_col})");
                match new_object {
                    Object::Robot => panic!("Robot pushing itself?"),
                    Object::Wall => {
                        // Any pending things can't actually move.
                        pending_box = None;
                        pending_robot = None;
                        break;
                    }
                    Object::Empty => {
                        if pending_robot.is_none() {
                            pending_robot = Some((pending_row, pending_col));
                        }
                        if pending_box.is_some() {
                            // We'll become a box...
                            // This is kinda weird.
                            pending_box = Some((pending_row, pending_col));
                        }
                        break;
                    }
                    Object::Box => {
                        if pending_robot.is_none() {
                            // pending_robot should be None iff it's the first time through.
                            // So while new_row / col will eventually follow a chain, it hasn't yet.
                            pending_robot = Some((pending_row, pending_col));
                        }
                        pending_box = Some((pending_row, pending_col));
                        // (pending_row, pending_col) = (new_row, new_col);
                        // we continue
                    }
                }
            }
            if let Some((next_row, next_col)) = pending_robot {
                // invert direction
                let (from_row, from_col) = match direction {
                    Move::U => (next_row + 1, next_col),
                    Move::D => (next_row - 1, next_col),
                    Move::L => (next_row, next_col + 1),
                    Move::R => (next_row, next_col - 1),
                };
                // eprintln!("Update robot! ({}, {}) -> ({}, {})", from_row, from_col, next_row, next_col);
                self.set(from_row, from_col, Object::Empty);
                self.set(next_row, next_col, Object::Robot);
                row = next_row;
                col = next_col;
            }

            if let Some(next_box) = pending_box {
                self.set(next_box.0, next_box.1, Object::Box);
            }
            // self.print();
        }
    }

    pub fn score(&self) -> u64 {
        let mut result: u64 = 0;
        let indexes: Vec<_> = self
            .grid
            .iter()
            .enumerate()
            .filter(|(_i, v)| **v == Object::Box)
            .map(|x| x.0)
            .collect();
        for index in indexes.iter() {
            let (row, col) = (index / self.n_cols, index % self.n_cols);
            result += 100 * row as u64 + col as u64;
        }

        result
    }

    pub fn print(&self) {
        let mut buf = String::new();

        for (index, value) in self.grid.iter().enumerate() {
            if index > 0 && index % self.n_cols == 0 {
                buf.push('\n');
            }
            // let (row, col) = (index / self.n_cols, index % self.n_cols);
            let c = match value {
                Object::Box => 'O',
                Object::Empty => '.',
                Object::Robot => '@',
                Object::Wall => '#',
            };
            buf.push(c);
        }

        eprintln!("{buf}");
    }
}

pub fn main(input: &str) -> u64 {
    let mut map = parse_input(input);
    map.step();
    map.score()
}

#[cfg(test)]
mod tests {
    use crate::d15::{parse_input, Move, Object};

    const INPUT_SMALL: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const INPUT: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_small() {
        let mut result = parse_input(INPUT_SMALL);

        assert_eq!(result.get(2, 2), Some(&Object::Robot));
        assert_eq!(result.get(2, 1), Some(&Object::Wall));

        result.print();
        result.step();
        // result.print();
        assert_eq!(result.score(), 2028);
    }

    #[test]
    fn test_example_1() {
        let mut result = parse_input(INPUT);
        assert_eq!(result.grid[0], Object::Wall);
        assert_eq!(result.moves[0], Move::L);
        assert_eq!(result.n_rows, 10);
        assert_eq!(result.n_cols, 10);
        result.step();

        assert_eq!(result.score(), 10092);
    }
}

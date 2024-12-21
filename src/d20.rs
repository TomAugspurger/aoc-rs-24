use std::collections::{HashMap, VecDeque};

type Point = (usize, usize);
type Points = Vec<Point>;

pub fn parse_input(input: &str) -> (Points, Point, Point) {
    let mut result = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' || c == 'S' || c == 'E' {
                result.push((row, col));
            }
            if c == 'S' {
                start = (row, col);
            } else if c == 'E' {
                end = (row, col)
            }
        }
    }
    (result, start, end)
}

pub fn neighbors_of(point: &Point, valid: &Points) -> Points {
    let neighbors = [
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
        (point.0, point.1 + 1),
    ];
    neighbors
        .iter()
        .filter(|n| valid.contains(n))
        .copied()
        .collect()
}

pub fn search(start: &Point, end: &Point, valid: &Points) -> (Points, HashMap<Point, usize>) {
    let mut queue = VecDeque::new();
    queue.push_back(*start);
    let mut visited = HashMap::new();

    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        let neighbors = neighbors_of(&point, valid);
        for neighbor in neighbors.iter() {
            if neighbor == end {
                visited.insert(*neighbor, point);
                break;
            }

            if !visited.contains_key(neighbor) {
                visited.insert(*neighbor, point);
                queue.push_back(*neighbor);
            }
        }
    }

    // now go back from the target to the start.
    let mut path = Vec::new();
    let mut node = *end;
    let mut distances = HashMap::with_capacity(valid.len());
    let mut distance = 0;
    while node != *start {
        node = visited[&node];
        path.push(node);
        distances.insert(node, distance);
        distance += 1;
    }

    path.reverse();
    (path, distances)
}

pub fn find_cheats(
    // valid: &Points,
    solution: &Points,
    distances: &HashMap<Point, usize>,
) -> HashMap<(Point, Point), usize> {
    /*
    Are there any cheats that move diagonally? Maybe, if you have
    something like
    ....#
    .#1.#
    .2#.#
    #####

    You might jump from 1 -> 2.

    This means the possible cheats are in a in a diamond
    around the center like

        0
       7|1
      6-.-2
       5|3
        4

    */
    let offsets = [
        (-2, 0),
        (-1, 1),
        (0, 2),
        (1, 1),
        (2, 0),
        (1, -1),
        (0, -2),
        (-1, -1),
    ];

    let mut result = HashMap::new();

    // Will distances always contain the new point?
    // i.e. will a cheat always move *from* a point in the solution *to* a point in the solution?

    for point in solution.iter() {
        for offset in offsets.iter() {
            let cheat = (
                point.0.checked_add_signed(offset.0),
                point.1.checked_add_signed(offset.1),
            );

            if let (Some(row), Some(col)) = cheat {
                if solution.contains(&(row, col)) {
                    let new_distance = distances.get(&(row, col)).unwrap();
                    let old_distance = distances.get(point).unwrap();
                    if &(new_distance + 2) < old_distance {
                        result.insert((*point, (row, col)), old_distance - new_distance - 2);
                    }
                }
            }
            // if valid.contains(&cheat) {
            //     // check if the path from point -> cheat -> point is shorter than point -> cheat
            //     // -> solution -> cheat -> point
            // }
        }
    }
    result
}


pub fn main(input: &str) -> usize {
    let (valid, start, end) = parse_input(input);
    let (path, distances) = search(&start, &end, &valid);
    let cheats = find_cheats(&path, &distances);
    cheats.values().filter(|x| **x > 100).count()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_example_1() {
        let (valid, start, end) = super::parse_input(INPUT);
        let (path, distances) = super::search(&start, &end, &valid);
        assert_eq!(path[..4], vec![(3, 1), (2, 1), (1, 1), (1, 2)]);
        assert_eq!(distances.get(&(3, 1)), Some(83).as_ref());

        let cheats = super::find_cheats(&path, &distances);
        // eprintln!("{:?}", cheats);
        assert_eq!(cheats.get(&((1, 7), (1, 9))), Some(&12));
        // assert_eq!(cheats.len(), 0);
    }
}

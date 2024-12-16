/*

Get through the maze. Turns are *very* expensive.

Are there pathological cases where an "extra" turn
is worth it?

*/
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, u64};

pub struct Map {
    pub valid: Vec<(usize, usize)>,
    pub current: (usize, usize),
    pub end: (usize, usize),
}

/*
Search for all(?) paths through the maze.

What tricks can we pull to avoid an exhaustive search.

1. With the cost of turns at 1,000, and the size of the
   grid much less than that, the number of turns will
   always(?) be the most important factor. So maybe
   we count the number of turns each thing is from
   the end?

   - But the number of turns isn't unique per point.
     And we can say which is "lower" until we know
     what direction we're coming from...

   - But if you *know* that you must approach the end
     from some direction, is there some kind of intermediate
     value theorem that says it doesn't matter which way you go?
*/

// pub fn search(valid: Vec<(usize, usize)>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
//     todo!()
// }

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

pub fn search(start: (usize, usize), end: (usize, usize), valid: Vec<(usize, usize)>)  -> (u64, Vec<Direction>) {
    // Rust uses a max heap, i.e. pop will return the highest value.
    // We want to minimize our score / cost here, we we generally want to pop the lowest value.
    // So all push operations to the queue should wrap the score in a Reverse.
    //
    // - initial score 0
    // - initial best score: u64::MAX

    let mut q: BinaryHeap<_> = BinaryHeap::new();
    let mut best_score = u64::MAX;
    let mut paths = Vec::new();
    let mut visited = HashMap::new();
    // let mut direction = Direction::E;

    q.push((Reverse(0u64), start, Direction::E, vec![]));

    let mut i = 0;
    while !q.is_empty() {
        // Score is the distance, including the cost of turns, to the end
        // position is the (row, col)
        // direction is the Direction we're facing
        let (score, position, direction, path) = q.pop().unwrap();

        i += 1;
        if i > 10000000 {
            break
        }

        if score.0 > best_score {
            // we can't beat our best score from here, so give up.
            break
        }

        visited.entry((position, direction)).or_insert(score.0);
        if visited.get(&(position, direction)).unwrap() < &score.0 {
            // We've already visited here facing this direction, but with a lower cost. Give up.
            continue
        }

        // eprintln!("i={i} score={score:?} position={position:?} direction={direction:?}");
        // We get to (13, 13), but only (try to) head E from there.


        if position == end {
            // eprintln!("{path:?}");
            // assert!(false);
            best_score = score.0;
            paths.push(path.clone());
        }

        // direction after turning right
        let right = match direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
        // direction after turning left
        let left = match direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        };

        let forward_position = match direction {
            Direction::N => (position.0 - 1, position.1),
            Direction::S => (position.0 + 1, position.1),
            Direction::E => (position.0, position.1 + 1),
            Direction::W => (position.0, position.1 - 1),
        };
     
        if valid.contains(&forward_position) { 
            q.push((Reverse(score.0 + 1), forward_position, direction, [path.clone(), vec![direction]].concat()));
        }
        q.push((Reverse(score.0 + 1000), position, left, [path.clone(), vec![left]].concat()));
        q.push((Reverse(score.0 + 1000), position, right, [path.clone(), vec![right]].concat()));


    }
    (best_score, paths[paths.len() - 1].clone())

}

pub fn neighbors_of(valid: &Vec<(usize, usize)>, current: (usize, usize)) -> Vec<&(usize, usize)> {
    // might want to sort this, by the minimum number of turns?
    // then we'll need to pass around a direction we're coming from.
    valid
        .iter()
        .filter(|(a, b)| {
            current == (a + 1, *b)
                || current == (a - 1, *b)
                || (current == (*a, b - 1))
                || (current == (*a, b + 1))
        })
        .collect()
}

pub fn parse_input(input: &str) -> Map {
    let mut valid = Vec::new();
    let mut current: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    for (r, line) in input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == 'S' {
                current = Some((r, c));
            } else if char == 'E' {
                end = Some((r, c));
            }
            if char != '#' {
                valid.push((r, c));
            }
        }
    }

    Map {
        valid,
        current: current.unwrap(),
        end: end.unwrap(),
    }
}

pub fn main(input: &str) -> u64 {
    let map = parse_input(input);
    // eprintln!("valid={:?}", map.valid);
    let (score, _path) = search(map.current, map.end, map.valid);
    // eprintln!("path={path:?}");
    score
}

#[cfg(test)]
mod tests {
    use crate::d16::parse_input;

    use super::main;

    const INPUT_1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT_2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_example_1() {
        let m = parse_input(INPUT_1);
        assert_eq!(m.current, (13, 1));
        assert_eq!(m.end, (1, 13));
        let result = main(INPUT_1);
        assert_eq!(result, 7036);
        // assert_eq!(result, 1);
    }

    #[test]
    fn test_example_2() {
        let result = main(INPUT_2);
        assert_eq!(result, 11048);
    }
}

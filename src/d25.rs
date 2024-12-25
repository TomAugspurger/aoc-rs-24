/*

Locks go down. Keys go up.

*/

type Lock = Vec<u8>;
type Key = Vec<u8>;

pub fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    input.split("\n\n").for_each(|group| {
        let is_lock = group.starts_with("#");

        if is_lock {
            let mut lock = vec![0; group.split_once("\n").unwrap().0.len()];

            for line in group.lines().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock[i] += 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = vec![5; group.split_once("\n").unwrap().0.len()];

            for line in group.lines().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if c == '.' {
                        key[i] -= 1;
                    }
                }
            }
            keys.push(key);
        }
        // let mut lines = group.lines();
        // let lock = lines.next().unwrap().as_bytes().to_vec();
        // let key = lines.next().unwrap().as_bytes().to_vec();

        // locks.push(lock);
        // keys.push(key);
    });

    (locks, keys)
}

pub fn main(input: &str) -> usize {
    let (locks, keys) = parse_input(input);
    let mut result = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(k, l)| k + l <= 5) {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_parse_input() {
        let (locks, keys) = parse_input(INPUT);
        assert_eq!(locks[0], vec![0, 5, 3, 4, 3]);
        assert_eq!(keys[0], vec![5, 0, 2, 1, 3]);
    }

    #[test]
    fn example_1() {
        let result = main(INPUT);
        assert_eq!(result, 3);
    }
}

use itertools::Itertools;
// https://adventofcode.com/2024/day/13
use regex;

pub const COST_A: u64 = 3;
pub const COST_B: u64 = 1;
pub const MAX_PUSHES: u64 = 100;

#[derive(Debug)]
pub struct Game {
    pub a_x: u64,
    pub a_y: u64,
    pub b_x: u64,
    pub b_y: u64,
    pub x: u64,
    pub y: u64,
}

pub fn parse_input(input: &str) -> Vec<Game> {
    let mut lines = input.lines();
    let mut games = Vec::new();

    //
    use regex::Regex;
    let button_re = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let price_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    while let Some(a_line) = lines.next() {
        let caps = button_re.captures(a_line).unwrap();
        let a_x = caps[1].parse().expect("Failed to parse 'x' from A line");
        let a_y = caps[2].parse().unwrap();
        let b_line = lines.next().unwrap();
        let price_line = lines.next().unwrap();
        let _blank_line = lines.next().unwrap_or_default();

        let caps = button_re.captures(b_line).unwrap();
        let b_x = caps[1].parse().unwrap();
        let b_y = caps[2].parse().unwrap();

        let caps = price_re.captures(price_line).unwrap();
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();

        games.push(Game {
            a_x,
            a_y,
            b_x,
            b_y,
            x,
            y,
        });
    }
    games
}


impl Game {
    pub fn solve(&self) -> Option<u64> {
        // Find the number of A and B pushes that minimize the cost.
        // I think this is integer programming: https://en.wikipedia.org/wiki/Integer_programming
        // I'm not sure whether the separate cost function matters, though it's a simple
        // transform of the other variables: C(a, b) = 3a + b

        // Let's try the naive approach first.

        // Step 1: figure out the valid A range, s.t. the x and y aren't too large.

        let feasible_a = (0..=100).filter(|n| { self.a_x * n <= self.x && self.a_y * n <= self.y});
        let feasible_b = (0..=100).filter(|n| { self.b_x * n <= self.x && self.b_y * n <= self.y});

        // Now consider the combinations that meet the conditions.

        let feasible: Vec<_> = feasible_a.cartesian_product(feasible_b).filter(|(na, nb)| {
            (na * self.a_x + nb * self.b_x == self.x) && (na * self.a_y + nb * self.b_y == self.y)
        }).collect();

        let costs: Vec<_> = feasible.iter().map(|(nx, ny)| {
            COST_A * nx + COST_B * ny
        }).collect();

        costs.iter().min().cloned()

        // let best = feasible.iter().min_by(|(tx, ty), (ux, uy)| {
        //     (COST_A * tx + COST_B * ty).cmp(&(COST_A * ux + COST_B * uy))
        // });
        // best.cloned()
    }
}


pub fn main(input: &str, offset: u64) -> u64 {
    let _ = offset;
    let games = parse_input(input);
    games.iter().filter_map(|g| g.solve()).sum()
}

#[cfg(test)]
mod tests {
    use crate::d13::main;

    const INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_example_1() {
        assert_eq!(main(INPUT, 0), 480);
    }
}

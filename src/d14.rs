use std::collections;

type State = Vec<((usize, usize), (i32, i32))>;

pub fn parse_input(input: &str) -> State {
    let mut result: Vec<_> = Vec::new();

    for line in input.lines() {
        let (p, v) = line.split_once(' ').expect("Missing position or velocity.");
        let (px, py) = p
            .trim_start_matches("p=")
            .split_once(",")
            .expect("Failed to parse position");
        let (vx, vy) = v
            .trim_start_matches("v=")
            .split_once(",")
            .expect("Failed to parse position");

        let npx = px.parse().unwrap();
        let npy = py.parse().unwrap();
        let vpx = vx.parse().unwrap();
        let vpy = vy.parse().unwrap();

        result.push(((npx, npy), (vpx, vpy)))
    }

    result
}

pub fn shift_n(
    p: &(usize, usize),
    v: &(i32, i32),
    width: usize,
    height: usize,
    n: i32,
) -> (usize, usize) {
    let x = ((p.0 as i32) + (n * v.0)).rem_euclid(width as i32) as usize;
    let y = ((p.1 as i32) + (n * v.1)).rem_euclid(height as i32) as usize;
    (x, y)
}

pub fn step_n(state: &State, width: usize, height: usize, n: i32) -> State {
    let mut next = Vec::with_capacity(state.len());

    for (position, velocity) in state.iter() {
        // special kind of wrapping addition
        let new_p = shift_n(position, velocity, width, height, n);
        next.push((new_p, *velocity));
    }

    next
}

pub fn format_grid(state: &State, width: usize, height: usize) {
    let mut counts = collections::HashMap::new();
    for (p, _v) in state.iter() {
        *counts.entry(p).or_insert(0) += 1;
    }

    let mut result = String::new();
    for i in 0..height {
        for j in 0..width {
            let v = counts.get(&(j, i));
            if let Some(c) = v {
                result.push(char::from_digit(*c, 10).unwrap());
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }

    eprintln!("{result}");
}

pub fn main(input: &str, n_iter: usize, width: usize, height: usize) -> u64 {
    let mut state = parse_input(input);
    state = step_n(&state, width, height, n_iter as i32);
    // for _i in 0..n_iter {
    //     state = step(&state, width, height);
    //     // format_grid(&state, width, height);
    // }

    let mut quadrants = [0; 4];
    let w2 = width / 2;
    let h2 = height / 2;

    for (p, _) in state.iter() {
        if p.0 < w2 && p.1 < h2 {
            // top-left
            quadrants[0] += 1;
        } else if p.0 < w2 && p.1 > h2 {
            // bottom-left
            quadrants[1] += 1;
        } else if p.0 > w2 && p.1 < h2 {
            // top-right
            quadrants[2] += 1;
        } else if p.0 > w2 && p.1 > h2 {
            // bottom-right
            quadrants[3] += 1;
        }
    }

    // format_grid(&state, width, height);

    quadrants.iter().product()
}

fn var(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mu: f64 = x.iter().sum::<f64>() / n;
    let d2: Vec<_> = x.iter().map(|v| (v - mu).powf(2.0)).collect();
    d2.iter().sum::<f64>() / n
}

pub fn find_tree(input: &str, width: usize, height: usize) -> u64 {
    /*
    wow: https://www.reddit.com/r/adventofcode/comments/1he0asr/comment/m1zzfsh/

    Basic idea is to find the t that minimizes the variance of the x's and y's.

    The x and y movements are independent of each other, so optimize each separately.
    Then find the cycle where those two line up.

    */

    let n_max = width.max(height);
    let state = parse_input(input);
    let mut xs: Vec<f64> = Vec::with_capacity(n_max);
    let mut ys: Vec<_> = Vec::with_capacity(n_max);

    for n in 0..=n_max {
        let staten = step_n(&state, width, height, n as i32);
        let x: Vec<_> = staten.iter().map(|x| x.0 .0 as f64).collect();
        xs.push(var(&x));

        let y: Vec<_> = staten.iter().map(|x| x.0 .1 as f64).collect();
        ys.push(var(&y));
    }

    // ys.iter().arg

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::d14::parse_input;

    use super::{main, step_n};

    const INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_example_1() {
        let result = main(INPUT, 100, 11, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_step() {
        let state = parse_input(INPUT);
        let width = 11;
        let height = 7;
        assert_eq!(step_n(&state, width, height, 1)[10].0, (4, 1));
        assert_eq!(step_n(&state, width, height, 2)[10].0, (6, 5));
        assert_eq!(step_n(&state, width, height, 3)[10].0, (8, 2));
        assert_eq!(step_n(&state, width, height, 4)[10].0, (10, 6));
        assert_eq!(step_n(&state, width, height, 5)[10].0, (1, 3));
    }
}

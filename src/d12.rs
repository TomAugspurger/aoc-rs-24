// use std::{
//     collections::{HashMap, HashSet},
//     vec,
// };

// use itertools::Itertools;

// // use crate::utils::Grid;

// pub struct Field {
//     pub plots: Vec<Plot>,
//     pub n_rows: usize,
//     pub n_cols: usize,

//     // need this as a way to avoid cycles while forming regions.
//     pub visited: Vec<(usize, usize)>,
// }

// pub struct Plot {
//     pub label: char,
//     pub row: usize,
//     pub col: usize,
//     // the root node?
//     pub region: Option<(usize, usize)>,
// }

// pub fn parse_input(input: &str) -> Field {
//     let mut plots = Vec::new();
//     let n_rows = input.matches("\n").count();
//     let mut n_cols = 0;
//     for (row, line) in input.lines().enumerate() {
//         n_cols = line.len();
//         for (col, label) in line.chars().enumerate() {
//             plots.push(Plot {
//                 label,
//                 row,
//                 col,
//                 region: None,
//             });
//         }
//     }
//     Field { plots, n_rows, n_cols, visited: Vec::new()}
// }


// impl Field {
//     fn get(&self, row: usize, col: usize) -> Option<&Plot> {
//         self.plots.get(row * self.n_cols + col)
//     }
// }

// pub fn form_groups(grid: &Field) -> Field {
//     let new_plots: Vec<Plot> = Vec::with_capacity(grid.plots.capacity());
//     for plot in grid.plots.iter() {
//         loop {
//             let neighbors = [
//                 (plot.row.checked_sub(1), Some(plot.col)),
//                 (plot.row.checked_add(1), Some(plot.col)),
//                 (Some(plot.row), plot.col.checked_sub(1)),
//                 (Some(plot.row), plot.col.checked_add(1)),
//             ];
//             // Cases to consider
//             // 1. Any neighbors with the same label and a region  => set our label to its
//             // 2. A neighbor with the same label, but unknown region => follow its neighbors (DFS).
//             // 3. *Zero* neighbors with the same label => start a new Region
//             let mut queue: = Vec::new();

//             // we also need to track if we've visited stuff.
//             for neighbor in neighbors {
//                 match neighbor {
//                     (Some(r), Some(c)) => {
//                         // TODO: maybe push the option to the label and compare that?
//                         let neighbor_plot = grid.get(r, c).unwrap();
//                         if plot.label == neighbor_plot.label {
//                             // We've found a neighbor.
//                             if let region = Some(neighbor_plot.region) {
//                                 // Our neighbor has a known region. Set our region to its and... break?
//                                 todo!()
//                             }
//                             else {
//                                 // our neighbor doesn't have a known region. Push it to the queue of stuff to follow.
//                                 queue.push(neighbor);
//                             }
//                         }
//                     }
//                     _ => {}
//                 }

//             }


//         }
//     }

//     todo!()
// }

// fn is_adjacent(row: usize, col: usize, region: &[(usize, usize)]) -> bool {
//     // A point is adjacent to us if we can move left (or up, right, down) by one
//     // and hit the point.
//     // how is (6, 3) adjacent to [(0, 4), (0, 5), (1, 4), (1, 5)]?
//     // None  , (1, 4), (0, 3), (0, 5)
//     // None  , (1, 5), (0, 4), (0, 6)
//     // (0, 4), (2, 4), (1, 3), (1, 5)
//     // (0, 5), (2, 5), (1, 4), (1, 6)

//     let touches: Vec<_> = region
//         .iter()
//         .map(|(r, c)| {
//             let xs = [
//                 (r.checked_sub(1), Some(*c)), // up
//                 (r.checked_add(1), Some(*c)), // down
//                 (Some(*r), c.checked_sub(1)), // left
//                 (Some(*r), c.checked_add(1)), // right
//             ];

//             // eprintln!("xs={xs:?}");

//             xs.contains(&(Some(row), Some(col)))
//         })
//         .collect();
//     // eprintln!("touches={touches:?}");

//     touches.iter().any(|x| *x)
// }

// pub fn neighbors_of(row: usize, col: usize) -> Vec<(usize, usize)> {
//     let neighbors = [
//         (row.checked_sub(1), Some(col)),
//         (row.checked_add(1), Some(col)),
//         (Some(row), col.checked_sub(1)),
//         (Some(row), col.checked_add(1)),
//     ];

//     neighbors
//         .iter()
//         .filter(|(r, c)| r.is_some() && c.is_some())
//         .map(|(r, c)| (r.unwrap(), c.unwrap()))
//         .collect()
// }

// pub fn merge_adjacent(a: &[(usize, usize)], b: &[(usize, usize)]) -> Option<Vec<(usize, usize)>> {
//     /*
//     Turns out, we can't actually determine regions one plot at a time. E.g.

//        ABA
//        BBB

//     If you iterate in order, you'll get to the 'B' at (1, 0) and see it's not
//     connected to the 'B' and (0, 1). But it actually is, via the 'B' at (1, 1).

//     We just don't have that yet.
//     */
//     // Get the unique neighbors of each point in a
//     let a_neighbors: HashSet<_> = a
//         .iter()
//         .flat_map(|(r, c)| neighbors_of(*r, *c))
//         .unique()
//         .collect();
//     if a_neighbors.is_disjoint(&HashSet::from_iter(b.iter().copied())) {
//         None
//     } else {
//         Some([a, b].concat())
//     }
// }

// pub fn merge(regions: &[Vec<(usize, usize)>]) -> Vec<Vec<(usize, usize)>> {
//     let mut n_regions = regions.len();
//     loop {
//         let mut result = Vec::new();
//         for pair in regions.iter().combinations(2) {
//             let maybe_merged = merge_adjacent(pair[0], pair[1]);
//             match maybe_merged {
//                 None => {
//                     result.push(pair[0].clone());
//                     result.push(pair[1].clone());
//                 }
//                 Some(merged) => result.push(merged),
//             }
//         }

//         result = result.iter().unique().collect_vec();

//         // result = result.iter().unique().collect();

//         if result.len() == n_regions {
//             break result;
//         } else {
//             n_regions = result.len();
//         }
//     }
// }

// pub fn main(input: &str) -> u64 {
//     let grid = parse_input(input);
//     // let mut regions: HashMap<(char, (usize, usize)), u32> = HashMap::new();
//     // We need some way to handle disjointed regions.
//     // Right now, I think we'll have a vector of (row, col).
//     // HashMap<&char, Vec<Vec<(usize, usize)>>;
//     let mut regions: HashMap<&char, Vec<Vec<(usize, usize)>>> = HashMap::new();

//     for (i, line) in grid.values.iter().enumerate() {
//         for (j, c) in line.iter().enumerate() {
//             let regions: &mut Vec<Vec<(usize, usize)>> = regions.entry(c).or_default();

//             // Check if we touch any existing regions. If so, we'll expand it.
//             // Otherwise, we'll add a new region.

//             let adjacent: Vec<_> = regions.iter().map(|x| is_adjacent(i, j, x)).collect();
//             eprintln!("char={c} row={i} col={j} regions={regions:?} is_adjacent={adjacent:?}");
//             let m = adjacent.iter().position(|x| *x);
//             match m {
//                 Some(index) => regions[index].push((i, j)),
//                 None => regions.push(vec![(i, j)]),
//             }
//         }
//     }

//     // We need to union these things. Dumb way: pairwise.

//     let mut regions2 = HashMap::new();

//     for (k, xs) in regions.iter() {
//         regions2.insert(k, merge(xs));
//     }

//     eprintln!("regions={regions:?}");
//     let mut result = 0;

//     for (c, lol) in regions2.iter() {
//         for region in lol.iter() {
//             let region_area = region.len() as u64;
//             // perimeter is the number of sides in the region not touching a plot of the same type.
//             let mut region_perimeter = 0;
//             let other = HashSet::from_iter(region.iter().cloned());

//             let rmin = region.iter().map(|r| r.0).min().unwrap();
//             let cmin = region.iter().map(|r| r.1).min().unwrap();
//             let rmax = region.iter().map(|r| r.0).max().unwrap();
//             let cmax = region.iter().map(|r| r.1).max().unwrap();

//             for (row, col) in region.iter() {
//                 let maybe_neighbors = [
//                     (row.checked_sub(1), Some(*col)),
//                     (row.checked_add(1), Some(*col)),
//                     (Some(*row), col.checked_sub(1)),
//                     (Some(*row), col.checked_add(1)),
//                 ];
//                 // char=B row=1 col=0 region=[(1, 0), (1, 1), (2, 0), (2, 1)] neighbors={(1, 1)}
//                 // maybe_neighbors=[, (Some(2), Some(0)), , (Some(1), Some(1))] intersection=[(1, 1)] rmin=1 rmax=2 cmin=0 cmax=1
//                 let neighbors: HashSet<_> = maybe_neighbors
//                     .iter()
//                     .filter(|(r, c)| {
//                         r.is_some()
//                             && c.is_some()
//                             && r.unwrap() >= rmin
//                             && r.unwrap() <= rmax
//                             && c.unwrap() >= cmin
//                             && c.unwrap() <= cmax
//                     })
//                     .map(|a| (a.0.unwrap(), a.1.unwrap()))
//                     .collect();
//                 let intersection: Vec<_> = neighbors.intersection(&other).collect();
//                 // eprintln!("char={c} row={row} col={col} region={region:?} neighbors={neighbors:?} maybe_neighbors={maybe_neighbors:?} intersection={intersection:?} rmin={rmin} rmax={rmax} cmin={cmin} cmax={cmax}");
//                 region_perimeter += 4 - intersection.len() as u64;
//             }

//             eprintln!("char={c} area={region_area} perimeter={region_perimeter}");
//             result += region_area * region_perimeter;
//         }
//     }
//     // regions

//     result
// }
// #[cfg(test)]
// mod tests {
//     use crate::d12::{is_adjacent, main, main2};

//     const INPUT1: &str = "\
// AAAA
// BBCD
// BBCC
// EEEC";

//     const INPUT2: &str = "\
// OOOOO
// OXOXO
// OOOOO
// OXOXO
// OOOOO";
//     const INPUT3: &str = "\
// RRRRIICCFF
// RRRRIICCCF
// VVRRRCCFFF
// VVRCCCJFFF
// VVVVCJJCFE
// VVIVCCJJEE
// VVIIICJJEE
// MIIIIIJJEE
// MIIISIJEEE
// MMMISSJEEE";

//     #[test]
//     fn test_example_1() {
//         assert_eq!(main2(INPUT1), 140);
//         // assert_eq!(main(INPUT2), 772);
//         // assert_eq!(main(INPUT3), 1930);
//     }

//     #[test]
//     fn test_adjacent() {
//         assert!(!is_adjacent(6, 3, &[(0, 4)]));
//     }
// }

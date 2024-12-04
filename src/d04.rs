/*
Strategy:

1. Find the "x"s
2. For each "x"
    - Find valid directions
    - Search in that direction

---

Alternate strategy:

1. Iterate over the characters, keeping track of possible matches.

..X...
.SAMX.
.A.*A.
XMAS.S
.X....

---


Alternate: just search forwards, backwards, and all the diagonals?

*/

pub fn generate_slices(input: &str) -> Vec<String> {
    /*

    1. lines forward
    2. lines reversed (or skip, and search samx)
    3. columns forward
    4. columns reversed (or skip, and search samx)
    5. diagonals NW to SE
    6. diagonals NE to SW

    */

    // TODO: figure out how to yield these?
    // TODO: we should know the capacity from input.
    let mut horizontal: Vec<String> = Vec::new();
    let mut vertical: Vec<String> = Vec::new();
    let mut diagonal_lr: Vec<String> = Vec::new();
    let mut diagonal_rl: Vec<String> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let n = line.len();

        // horizontal (reversible). Append each line.
        horizontal.push(String::from(line));

        for (j, c) in line.chars().enumerate() {
            // vertical (reversible).
            if i == 0 {
                vertical.push(String::from(c));
            } else {
                vertical[j].push(c);
            }

            // handle all the LR first

            // diagonal.
            // There are M + (N - 1) diagonals in an (MxN) matrix
            if i == 0 || j == 0 {
                diagonal_lr.push(String::from(c));
            } else if j >= i {
                // Extend a diagonal that started on the first row
                diagonal_lr[j - i].push(c);
            } else {
                // Extend a diagonal that started on the i-th row.
                // For example, at position [2, 1] (row 3, col 2) of a 3x4 matrix
                // we have i=2, j=1, n=4. Our string is at position 4 in the vector.
                // i - j + n - 1 = 2 - 1 + 4 - 1 = 4.
                // See if there's a general formula?
                diagonal_lr[i - j + n - 1].push(c);
            }

            // Handle all the RL
            // eprintln!("i={i} j={j} n={n}");
            if i == 0 || j == (n - 1) {
                // create a new diagonal start on ...
                diagonal_rl.push(String::from(c));
            } else {
                diagonal_rl[i + j].push(c);
            }
        }
    }

    // horizontal.iter().chain(vertical).it
    [horizontal, vertical, diagonal_lr, diagonal_rl].concat()
}

pub fn count_mas_xs(input: &str) -> usize {
    // Look in the window around us
    let mut count = 0;
    let lines: Vec<_> = input.lines().collect();
    let windows = lines.windows(3);
    let n = lines[0].len();

    for window in windows {
        for (j, char) in window[1].chars().enumerate() {
            if char == 'A' && (1..n - 1).contains(&j) {
                let a = window[0].as_bytes()[j - 1]; // M / 77
                let b = window[0].as_bytes()[j + 1]; // S / 83
                let c = window[2].as_bytes()[j - 1];
                let d = window[2].as_bytes()[j + 1];

                let xx = (a == b'M' && d == b'S') || (a == b'S' && d == b'M');
                let yy = (b == b'M' && c == b'S') || (b == b'S' && c == b'M');

                if xx && yy {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn count_xmas(input: &str, xs: bool) -> usize {
    if xs {
        count_mas_xs(input)
    } else {
        let sequences = generate_slices(input);
        // eprintln!("{sequences:#?}");
        let mut count = 0;
        for sequence in sequences {
            count += sequence.matches("XMAS").count();
            count += sequence.matches("SAMX").count();
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::d04::count_xmas;

    #[test]
    fn test_part_1_example() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = count_xmas(input, false);
        assert_eq!(result, 18);
        let result = count_xmas(input, true);
        assert_eq!(result, 9);
    }
}

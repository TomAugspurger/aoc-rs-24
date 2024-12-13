pub struct Grid<T> {
    pub values: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn n_rows(&self) -> usize {
        self.values.len()
    }

    pub fn n_cols(&self) -> usize {
        self.values[0].len()
    }

    pub fn up(&self, row: usize, col: usize) -> Option<&T> {
        self.iloc(row - 1, col)
    }

    pub fn down(&self, row: usize, col: usize) -> Option<&T> {
        self.iloc(row + 1, col)
    }

    pub fn left(&self, row: usize, col: usize) -> Option<&T> {
        self.iloc(row, col - 1)
    }
    pub fn right(&self, row: usize, col: usize) -> Option<&T> {
        self.iloc(row, col + 1)
    }

    pub fn iloc(&self, row: usize, col: usize) -> Option<&T> {
        self.values.get(row)?.get(col)
    }
    //     eprintln!("row={row:} col={col}");
    //     self.values.get(0).or_else(|r| r.get)
    //     if (row >= self.n_rows()) || (col == 0 || col == self.n_cols() - 1) {
    //         None
    //     } else {
    //         Some(self.values[row][col])
    //     }
    // }

    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let choices = [
            (row.checked_sub(1), Some(col)),
            (row.checked_add(1), Some(col)),
            (Some(row), col.checked_sub(1)),
            (Some(row), col.checked_add(1)),
        ];
        choices
            .iter()
            .filter(|(r, c)| {
                r.is_some()
                    && c.is_some()
                    && r.unwrap() < self.n_rows()
                    && c.unwrap() < self.n_cols()
            })
            .map(|a| (a.0.unwrap(), a.1.unwrap()))
            .collect()
    }
}


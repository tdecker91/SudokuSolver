use std::fmt;

pub mod candidates;
pub mod solver;

pub struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(puzzle: Vec<u8>) -> Sudoku {
        let mut grid = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                grid[i][j] = puzzle[i * 9 + j];
            }
        }
        Sudoku { grid }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.grid[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.grid[row][col] = value;
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col] == 0
    }

    pub fn is_solved(&self) -> bool {
        self.check_rows() && self.check_cols() && self.check_boxes()
    }

    fn check_rows(&self) -> bool {
        self.grid.iter().all(|row| Self::is_valid_set(row))
    }

    fn check_cols(&self) -> bool {
        (0..9).all(|col| {
            let column: Vec<u8> = self.grid.iter().map(|row| row[col]).collect();
            Self::is_valid_set(&column)
        })
    }

    fn check_boxes(&self) -> bool {
        (0..3).all(|box_row| {
            (0..3).all(|box_col| {
                let box_values: Vec<u8> = (0..3)
                    .flat_map(|row| {
                        (0..3).map(move |col| self.grid[box_row * 3 + row][box_col * 3 + col])
                    })
                    .collect();
                Self::is_valid_set(&box_values)
            })
        })
    }

    fn is_valid_set(set: &[u8]) -> bool {
        let mut seen = [false; 9];
        set.iter().all(|&num| {
            if num == 0 || seen[num as usize - 1] {
                false
            } else {
                seen[num as usize - 1] = true;
                true
            }
        })
    }
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sudoku:\n{}", self)
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.grid.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, &cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    write!(f, "| ")?;
                }
                if cell == 0 {
                    write!(f, ". ")?;
                } else {
                    write!(f, "{} ", cell)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Sudoku {
            grid: self.grid.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sudoku() {
        let puzzle = vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];
        let sudoku = Sudoku::new(puzzle);
        assert_eq!(sudoku.get(0, 0), 5);
        assert_eq!(sudoku.get(0, 2), 0);
        assert!(sudoku.is_empty(0, 2));
        assert!(!sudoku.is_empty(0, 0));
    }
}

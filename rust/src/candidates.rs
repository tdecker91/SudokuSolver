use crate::Sudoku;
use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct Candidates {
    pub grid: HashMap<(usize, usize), HashSet<u8>>,
}

impl Candidates {
    pub fn new() -> Self {
        let mut grid = HashMap::new();
        for row in 0..9 {
            for col in 0..9 {
                grid.insert((row, col), HashSet::new());
            }
        }
        Candidates { grid }
    }

    pub fn add(&mut self, row: usize, col: usize, value: u8) {
        self.grid
            .entry((row, col))
            .or_insert_with(HashSet::new)
            .insert(value);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&HashSet<u8>> {
        self.grid.get(&(row, col))
    }

    pub fn calculate(sudoku: &Sudoku) -> Self {
        let mut candidates = Self::new();

        for row in 0..9 {
            for col in 0..9 {
                if sudoku.is_empty(row, col) {
                    let values = Candidates::find_possible_values(sudoku, row, col);
                    candidates.grid.insert((row, col), values);
                } else {
                    candidates.grid.insert((row, col), HashSet::new());
                }
            }
        }

        candidates
    }

    fn find_possible_values(sudoku: &Sudoku, row: usize, col: usize) -> HashSet<u8> {
        let mut values: HashSet<u8> = (1..=9).collect();

        // Remove values from the same row
        for c in 0..9 {
            values.remove(&sudoku.get(row, c));
        }

        // Remove values from the same column
        for r in 0..9 {
            values.remove(&sudoku.get(r, col));
        }

        // Remove values from the same 3x3 box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                values.remove(&sudoku.get(r, c));
            }
        }

        values
    }
}

impl fmt::Debug for Candidates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Candidates {{")?;
        for ((row, col), candidates) in &self.grid {
            writeln!(f, "    ({}, {}): {:?}", row, col, candidates)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use crate::Sudoku;
    use crate::candidates::Candidates;

    #[test]
    fn test_candidates_new() {
        let candidates = Candidates::new();
        assert_eq!(candidates.grid.len(), 81);

        for row in 0..9 {
            for col in 0..9 {
                assert_eq!(candidates.grid.get(&(row, col)).unwrap().len(), 0);
            }
        }
    }

    #[test]
    fn test_candidates_get() {
        let candidates = Candidates::new();
        assert_eq!(candidates.get(0, 0).unwrap().len(), 0);
    }

    #[test]
    fn test_candidates_add() {
        let mut candidates = Candidates::new();

        // Test adding a single candidate
        candidates.add(0, 0, 1);
        assert_eq!(
            candidates.get(0, 0).expect("Cell (0,0) should exist").len(),
            1
        );
        assert!(candidates.get(0, 0).unwrap().contains(&1));

        // Test adding multiple candidates to the same cell
        candidates.add(0, 0, 2);
        candidates.add(0, 0, 3);
        assert_eq!(
            candidates.get(0, 0).expect("Cell (0,0) should exist").len(),
            3
        );
        assert!(candidates.get(0, 0).unwrap().contains(&1));
        assert!(candidates.get(0, 0).unwrap().contains(&2));
        assert!(candidates.get(0, 0).unwrap().contains(&3));

        // Test adding candidates to different cells
        candidates.add(1, 2, 4);
        candidates.add(1, 2, 5);
        assert_eq!(
            candidates.get(1, 2).expect("Cell (1,2) should exist").len(),
            2
        );
        assert!(candidates.get(1, 2).unwrap().contains(&4));
        assert!(candidates.get(1, 2).unwrap().contains(&5));

        // Verify original cell still has its candidates
        assert_eq!(candidates.get(0, 0).unwrap().len(), 3);
    }

    #[test]
    fn test_calculate() {
        let puzzle = vec![
            0, 3, 5, 2, 6, 9, 7, 8, 1, 6, 8, 2, 5, 7, 1, 4, 9, 3, 1, 9, 7, 8, 3, 4, 5, 6, 2, 8, 2,
            6, 1, 9, 5, 3, 4, 7, 3, 7, 4, 6, 8, 2, 9, 1, 5, 9, 5, 1, 7, 4, 3, 6, 2, 8, 5, 1, 9, 3,
            2, 6, 8, 7, 4, 2, 4, 8, 9, 5, 7, 1, 3, 6, 7, 6, 3, 4, 1, 8, 2, 5, 9,
        ];
        let sudoku = Sudoku::new(puzzle);
        let candidates = Candidates::calculate(&sudoku);

        let values = candidates.get(0, 0).expect("Cell (0,0) should exist");
        assert_eq!(values.len(), 1);
        assert!(values.contains(&4));

        let values = candidates.get(0, 1).expect("Cell (0,1) should exist");
        assert_eq!(values.len(), 0);
    }
}

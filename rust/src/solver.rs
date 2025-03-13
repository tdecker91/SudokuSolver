use crate::Sudoku;
use crate::candidates::Candidates;

pub fn solve(sudoku: &Sudoku) -> Option<Sudoku> {
    let candidates = Candidates::calculate(sudoku);
    solve_recursive(sudoku, &candidates)
}

fn solve_recursive(sudoku: &Sudoku, candidates: &Candidates) -> Option<Sudoku> {
    if sudoku.is_solved() {
        return Some(sudoku.clone());
    }

    if let Some((row, col, values)) = find_best_candidate(sudoku, candidates) {
        for &value in values.iter() {
            // change to stack method instead of cloning
            let mut new_sudoku = sudoku.clone();
            new_sudoku.set(row, col, value);
            let new_candidates = Candidates::calculate(&new_sudoku);
            if let Some(solved) = solve_recursive(&new_sudoku, &new_candidates) {
                return Some(solved);
            }
        }
    }

    None
}

fn find_best_candidate<'a>(
    sudoku: &Sudoku,
    candidates: &'a Candidates,
) -> Option<(usize, usize, &'a std::collections::HashSet<u8>)> {
    let mut best: Option<(usize, usize, &std::collections::HashSet<u8>)> = None;
    let mut min_candidates = 10;

    for row in 0..9 {
        for col in 0..9 {
            if sudoku.is_empty(row, col) {
                if let Some(values) = candidates.get(row, col) {
                    if values.iter().count() < min_candidates {
                        min_candidates = values.iter().count();
                        best = Some((row, col, values));
                    }
                }
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sudoku;

    #[test]
    fn test_solved_puzzle() {
        let puzzle = vec![
            4, 3, 5, 2, 6, 9, 7, 8, 1, 6, 8, 2, 5, 7, 1, 4, 9, 3, 1, 9, 7, 8, 3, 4, 5, 6, 2, 8, 2,
            6, 1, 9, 5, 3, 4, 7, 3, 7, 4, 6, 8, 2, 9, 1, 5, 9, 5, 1, 7, 4, 3, 6, 2, 8, 5, 1, 9, 3,
            2, 6, 8, 7, 4, 2, 4, 8, 9, 5, 7, 1, 3, 6, 7, 6, 3, 4, 1, 8, 2, 5, 9,
        ];

        let sudoku = Sudoku::new(puzzle);
        let solution = solve(&sudoku);

        assert!(solution.is_some(), "Expected to find a solution")
    }

    #[test]
    fn test_almost_solved_puzzle() {
        let puzzle = vec![
            0, 3, 5, 2, 6, 9, 7, 8, 1, 6, 8, 2, 5, 7, 1, 4, 9, 3, 1, 9, 7, 8, 3, 4, 5, 6, 2, 8, 2,
            6, 1, 9, 5, 3, 4, 7, 3, 7, 4, 6, 8, 2, 9, 1, 5, 9, 5, 1, 7, 4, 3, 6, 2, 8, 5, 1, 9, 3,
            2, 6, 8, 7, 4, 2, 4, 8, 9, 5, 7, 1, 3, 6, 7, 6, 3, 4, 1, 8, 2, 5, 0,
        ];

        let sudoku = Sudoku::new(puzzle);
        let solution = solve(&sudoku);

        assert!(solution.is_some(), "Expected to find a solution");

        if let Some(solved) = solution {
            assert!(solved.is_solved());
            assert_eq!(solved.get(8, 8), 9);
            assert_eq!(solved.get(0, 0), 4);
        }
    }

    #[test]
    fn test_puzzle() {
        let puzzle = vec![
            6, 0, 8, 0, 2, 0, 0, 7, 0, 0, 4, 0, 0, 0, 0, 0, 8, 0, 0, 0, 3, 0, 0, 0, 2, 0, 5, 8, 0,
            2, 0, 4, 1, 0, 0, 0, 0, 6, 4, 2, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            6, 0, 1, 0, 3, 0, 5, 0, 0, 0, 0, 0, 4, 6, 0, 0, 0, 0, 1, 0, 0, 5, 0,
        ];

        let sudoku = Sudoku::new(puzzle);
        let solution = solve(&sudoku);

        assert!(solution.is_some(), "Expected to find a solution");

        if let Some(solved) = solution {
            assert!(solved.is_solved());
            assert_eq!(solved.get(0, 1), 1);
            assert_eq!(solved.get(8, 0), 3);
        }
    }

    #[test]
    fn test_no_solution() {
        let puzzle = vec![
            6, 0, 1, 5, 9, 0, 0, 0, 0, 0, 9, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 7,
            0, 3, 1, 4, 0, 0, 6, 0, 2, 4, 0, 0, 0, 0, 0, 5, 0, 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 6, 0,
            0, 0, 0, 0, 3, 0, 0, 0, 9, 0, 2, 0, 4, 0, 0, 0, 0, 0, 0, 1, 6, 0, 0,
        ];

        let sudoku = Sudoku::new(puzzle);
        let solution = solve(&sudoku);

        assert!(solution.is_none(), "Expected None for unsolvable puzzle");
    }
}

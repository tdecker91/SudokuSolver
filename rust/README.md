# Sudoku Solver Rust

A Sudoku solver implemented in Rust.

## Example (Rust)

```rust
use sudoku_solver::Sudoku;
use sudoku_solver::solve;

fn main() {
    let puzzle = vec![
        5, 3, 0, 0, 7, 0, 0, 0, 0,
        6, 0, 0, 1, 9, 5, 0, 0, 0,
        0, 9, 8, 0, 0, 0, 0, 6, 0,
        8, 0, 0, 0, 6, 0, 0, 0, 3,
        4, 0, 0, 8, 0, 3, 0, 0, 1,
        7, 0, 0, 0, 2, 0, 0, 0, 6,
        0, 6, 0, 0, 0, 0, 2, 8, 0,
        0, 0, 0, 4, 1, 9, 0, 0, 5,
        0, 0, 0, 0, 8, 0, 0, 7, 9
    ];
    let sudoku = Sudoku::new(puzzle);
    if let Some(solution) = solve(&sudoku) {
        println!("Solved Sudoku:\n{}", solution);
    } else {
        println!("No solution exists.");
    }
}
```


## Implementation Details
- Uses a backtracking algorithm to solve the Sudoku puzzle
- Implements efficient candidate selection to improve solving speed
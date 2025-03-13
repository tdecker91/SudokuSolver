# Sudoku Solver
```javascript
. . . | 9 2 8 | 1 . .
. 8 . | 7 . 3 | . 2 4
2 6 . | . . 5 | . 7 .
------+-------+------
. 9 . | 1 . 7 | 2 . .
. 1 . | . . 6 | . 4 7
3 . . | . 8 . | . 9 1
------+-------+------
. 2 8 | 5 4 . | . . 6
. . 6 | 8 3 1 | 7 5 .
. . 5 | . 7 . | . . .
```

## Description
This project implements a Sudoku solver in different languages. It uses a combination of constraint propagation and backtracking to efficiently solve Sudoku puzzles of varying difficulty.

This solver stops as soon as it finds a single solution and does not determine if there are multiple solutions for the given sudoku puzzle.

## Features
- Solves 9x9 Sudoku puzzles
- Uses efficient algorithms for constraint propagation
- Implements backtracking for complex puzzles

## Project structure
```
sudoku_solver/ 
  ├── elixir/ # Elixir implementation 
  ├── rust/   # Rust implementation 
```
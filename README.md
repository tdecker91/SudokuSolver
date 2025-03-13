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
This project implements a Sudoku solver in Elixir. It uses a combination of constraint propagation and backtracking to efficiently solve Sudoku puzzles of varying difficulty.

This solver stops as soon as it finds a single solution and does not determine if there are multiple solutions for the given sudoku puzzle.

## Features
- Solves 9x9 Sudoku puzzles
- Uses efficient algorithms for constraint propagation
- Implements backtracking for complex puzzles
- Provides a clean, functional implementation in Elixir

## Installation
To use this Sudoku solver, you need to have Elixir installed on your system. Then, follow these steps:

## Usage
To solve a Sudoku puzzle:

1. Open an Elixir interactive shell in the project directory:
> iex -S mix

2. Call the solver function with your Sudoku puzzle:
```elixir
> puzzle = [
  [0, 2, 0, 6, 0, 9, 0, 3, 8],
  [0, 1, 9, 3, 2, 0, 0, 0, 0],
  [0, 6, 0, 1, 0, 0, 5, 2, 9],
  [0, 0, 0, 0, 4, 0, 8, 1, 6],
  [4, 7, 0, 0, 5, 6, 2, 0, 0],
  [6, 0, 3, 9, 1, 0, 0, 0, 0],
  [1, 9, 5, 0, 6, 8, 0, 0, 0],
  [7, 0, 0, 0, 0, 1, 4, 0, 5],
  [0, 0, 8, 0, 0, 7, 0, 0, 0]
]
> sudoku = Sudoku.new(puzzle)
> solution = Solver.solve(sudoku)
> IO.puts(solution)
```

```javascript
5 2 4 | 6 7 9 | 1 3 8
8 1 9 | 3 2 5 | 6 4 7
3 6 7 | 1 8 4 | 5 2 9
------+-------+------
9 5 2 | 7 4 3 | 8 1 6
4 7 1 | 8 5 6 | 2 9 3
6 8 3 | 9 1 2 | 7 5 4
------+-------+------
1 9 5 | 4 6 8 | 3 7 2
7 3 6 | 2 9 1 | 4 8 5
2 4 8 | 5 3 7 | 9 6 1
```

## Implementation Details
The solver uses the following key components:

- `Sudoku` module: Represents the Sudoku board and provides utility functions
- `Candidates` module: Calculates possible values for each cell
- `Solver` module: Implements the main solving algorithm
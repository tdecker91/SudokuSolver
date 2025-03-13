# Sudoku Solver Elixir

## Example (Elixir)
1. Navigate to the `elixir` directory and run:

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
> ...
> sudoku = Sudoku.new(puzzle)
> ...
> solution = Solver.solve(sudoku)
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
defmodule SolverTest do
  use ExUnit.Case
  doctest Solver

  describe "solved?/1" do
    test "returns true for a solved puzzle" do
      assert Solver.solved?(Sudoku.solved())
    end

    test "returns false for unfinished puzzle" do
      refute Solver.solved?(Sudoku.easy())
    end

    test "returns false for a puzzle with a mistake" do
      refute Solver.solved?(Sudoku.new(Boards.finished_unsolved()))
    end
  end

  describe "solve/1" do
    test "returns a solved puzzle" do
      assert %Sudoku{board: solved} = Solver.solve(Sudoku.easy())

      solved_list =
        solved
        |> :array.to_list()
        |> Enum.map(&:array.to_list/1)

      assert [
               [5, 2, 4, 6, 7, 9, 1, 3, 8],
               [8, 1, 9, 3, 2, 5, 6, 4, 7],
               [3, 6, 7, 1, 8, 4, 5, 2, 9],
               [9, 5, 2, 7, 4, 3, 8, 1, 6],
               [4, 7, 1, 8, 5, 6, 2, 9, 3],
               [6, 8, 3, 9, 1, 2, 7, 5, 4],
               [1, 9, 5, 4, 6, 8, 3, 7, 2],
               [7, 3, 6, 2, 9, 1, 4, 8, 5],
               [2, 4, 8, 5, 3, 7, 9, 6, 1]
             ] = solved_list
    end

    test "returns no solution for an unsolvable puzzle" do
      assert :no_solution = Solver.solve(Sudoku.new(Boards.unsolvable_board()))
      assert :no_solution = Solver.solve(Sudoku.new(Boards.duplicate_given_box()))
      assert :no_solution = Solver.solve(Sudoku.new(Boards.duplicate_given_column()))
      assert :no_solution = Solver.solve(Sudoku.new(Boards.duplicate_given_row()))
      assert :no_solution = Solver.solve(Sudoku.new(Boards.finished_unsolved()))
    end
  end
end

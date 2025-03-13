defmodule SudokuTest do
  use ExUnit.Case
  doctest Sudoku

  describe "new/1" do
    test "creates a new sudoku puzzle" do
      result = Sudoku.new()
      assert %Sudoku{} = result
    end
  end

  describe "at/2" do
    setup do
      %{
        sudoku: Sudoku.solved()
      }
    end

    test "returns the value at the given position", %{sudoku: sudoku} do
      assert 4 == Sudoku.at(sudoku, 0, 0)
      assert 9 == Sudoku.at(sudoku, 8, 8)
      assert 5 == Sudoku.at(sudoku, 3, 5)
    end
  end

  describe "set/3" do
    setup do
      %{
        sudoku: Sudoku.solved()
      }
    end

    test "sets the value at the given position", %{sudoku: sudoku} do
      new_sudoku = Sudoku.set(sudoku, 0, 0, 6)

      assert 6 == Sudoku.at(new_sudoku, 0, 0)
      assert 4 == Sudoku.at(sudoku, 0, 0)
    end
  end
end

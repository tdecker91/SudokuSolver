defmodule Candidates do
  def calculate(%Sudoku{} = sudoku) do
    Enum.reduce(0..8, %{}, fn row, candidates ->
      Enum.reduce(0..8, candidates, fn col, acc ->
        values = find_possible_values(sudoku, row, col)
        Map.put(acc, {row, col}, {values, Enum.count(values)})
      end)
    end)
  end

  def find_possible_values(sudoku, row, col) do
    find_possible_values(sudoku, Sudoku.at(sudoku, row, col), row, col)
  end

  def find_possible_values(sudoku, 0, row, col) do
    all_values = MapSet.new(1..9)
    used_values = MapSet.new()

    # check row
    used_values =
      Enum.reduce(0..8, used_values, fn c, acc ->
        MapSet.put(acc, Sudoku.at(sudoku, row, c))
      end)

    # check col
    used_values =
      Enum.reduce(0..8, used_values, fn r, acc ->
        MapSet.put(acc, Sudoku.at(sudoku, r, col))
      end)

    # check 3x3 box
    box_row = div(row, 3) * 3
    box_col = div(col, 3) * 3

    used_values =
      Enum.reduce(0..2, used_values, fn r, acc ->
        Enum.reduce(0..2, acc, fn c, inner_acc ->
          MapSet.put(inner_acc, Sudoku.at(sudoku, box_row + r, box_col + c))
        end)
      end)

    MapSet.difference(all_values, used_values)
    |> MapSet.delete(0)
    |> MapSet.to_list()
  end

  def find_possible_values(_sudoku, _current_value, _row, _col) do
    # no possible values because the current value is not 0
    []
  end
end

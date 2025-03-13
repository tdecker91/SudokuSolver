defmodule Solver do
  @moduledoc """
  Solves Sudoku puzzles
  """

  def solve(sudoku) do
    sudoku
    |> Candidates.calculate()
    |> next_best_candidate()
    |> attempt_solve(sudoku)
  end

  defp attempt_solve({nil, nil, []}, sudoku) do
    if solved?(sudoku), do: sudoku, else: :no_solution
  end

  defp attempt_solve({row, col, values}, sudoku) do
    Enum.reduce_while(values, sudoku, fn value, _ ->
      sudoku
      |> Sudoku.set(row, col, value)
      |> solve()
      |> case do
        :no_solution ->
          {:cont, :no_solution}

        %Sudoku{} = solved_sudoku ->
          {:halt, solved_sudoku}
      end
    end)
  end

  def solved?(sudoku) do
    check_rows(sudoku) and
      check_cols(sudoku) and
      check_boxes(sudoku)
  end

  defp check_cells(%Sudoku{} = sudoku, direction) do
    access_fn =
      case direction do
        :row ->
          fn sudoku, row, i -> Sudoku.at(sudoku, row, i) end

        :col ->
          fn sudoku, col, i -> Sudoku.at(sudoku, i, col) end

        :box ->
          fn sudoku, box, i ->
            row = Kernel.div(box, 3) * 3 + Kernel.div(i, 3)
            col = Kernel.rem(box, 3) * 3 + Kernel.rem(i, 3)
            Sudoku.at(sudoku, row, col)
          end
      end

    Enum.all?(0..8, fn line ->
      Enum.reduce_while(0..8, %{}, fn i, numbers ->
        val = access_fn.(sudoku, line, i)

        cond do
          val == 0 -> {:halt, false}
          Map.has_key?(numbers, val) -> {:halt, false}
          true -> {:cont, Map.put(numbers, val, true)}
        end
      end)
    end)
  end

  defp check_rows(sudoku), do: check_cells(sudoku, :row)
  defp check_cols(sudoku), do: check_cells(sudoku, :col)
  defp check_boxes(sudoku), do: check_cells(sudoku, :box)

  defp next_best_candidate(candidates) do
    candidates
    |> Enum.filter(fn {_, {_, length}} -> length > 0 end)
    |> Enum.min_by(fn {_, {_, length}} -> length end, fn -> nil end)
    |> case do
      nil -> {nil, nil, []}
      {{row, col}, {values, _}} -> {row, col, values}
    end
  end
end

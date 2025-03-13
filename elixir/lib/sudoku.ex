defmodule Sudoku do
  @moduledoc """
  Represents a Sudoku puzzle state
  """
  import Boards

  defstruct board: :array.new(9, default: :array.new(9, default: 0))

  @type t :: %__MODULE__{
          board: :array.array(:array.array(integer()))
        }

  def new(board \\ default_board())

  def new(board) when is_list(board) do
    board
    |> Enum.map(&:array.from_list/1)
    |> :array.from_list()
    |> new()
  end

  def new(board) do
    %__MODULE__{board: board}
  end

  def default, do: %Sudoku{}

  def easy, do: new(easy_board())

  def hard, do: new(hard_board())

  def solved, do: new(solved_board())

  def unsolvable, do: new(unsolvable_board())

  def at(_, row, col)
      when row < 0 or
             row > 8 or
             col < 0 or
             col > 8,
      do: nil

  def at(%Sudoku{board: board} = _sudoku, row, col) do
    r = :array.get(row, board)
    :array.get(col, r)
  end

  def set(%Sudoku{board: board} = sudoku, row, col, val) do
    r = :array.get(row, board)
    r = :array.set(col, val, r)
    board = :array.set(row, r, board)
    %Sudoku{sudoku | board: board}
  end
end

defimpl String.Chars, for: Sudoku do
  def to_string(%Sudoku{board: board}) do
    Enum.map_join(0..8, "\n", fn row ->
      row_data =
        Enum.map(0..8, fn col ->
          r = :array.get(row, board)
          val = :array.get(col, r)

          case val do
            0 -> "."
            _ -> Integer.to_string(val)
          end
        end)

      # insert column separators
      row_data =
        row_data
        |> Enum.chunk_every(3)
        |> Enum.map(&Enum.join(&1, " "))
        |> Enum.join(" | ")

      # insert row separators
      if row == 2 || row == 5 do
        row_data <> "\n------+-------+------"
      else
        row_data
      end
    end)
  end
end

defimpl Inspect, for: Sudoku do
  def inspect(%Sudoku{} = sudoku, _opts) do
    to_string(sudoku)
  end
end

defmodule CandidatesTest do
  use ExUnit.Case
  doctest Candidates

  describe "calculate/1" do
    test "calculates all possible candidates from basic elimination" do
      sudoku = Sudoku.easy()
      candidates = Candidates.calculate(sudoku)

      assert %{
               {4, 5} => {[], 0},
               {1, 2} => {[], 0},
               {8, 5} => {[], 0},
               {8, 6} => {[1, 3, 6, 9], 4},
               {5, 2} => {[], 0},
               {3, 6} => {[], 0},
               {2, 4} => {[7, 8], 2},
               {4, 8} => {[3], 1},
               {6, 5} => {[], 0},
               {0, 3} => {[], 0},
               {1, 1} => {[], 0},
               {4, 3} => {[8], 1},
               {3, 7} => {[], 0},
               {5, 0} => {[], 0},
               {0, 5} => {[], 0},
               {0, 1} => {[], 0},
               {4, 0} => {[], 0},
               {3, 2} => {[2], 1},
               {8, 1} => {[3, 4], 2},
               {7, 3} => {[2], 1},
               {0, 8} => {[], 0},
               {3, 1} => {[5], 1},
               {6, 1} => {[], 0},
               {2, 0} => {[3, 8], 2},
               {8, 3} => {[2, 4, 5], 3},
               {8, 4} => {[3, 9], 2},
               {2, 7} => {[], 0},
               {4, 6} => {[], 0},
               {6, 2} => {[], 0},
               {0, 7} => {[], 0},
               {7, 2} => {[2, 6], 2},
               {0, 0} => {[5], 1},
               {8, 7} => {[6, 9], 2},
               {5, 8} => {[4, 7], 2},
               {7, 6} => {[], 0},
               {2, 8} => {[], 0},
               {1, 4} => {[], 0},
               {5, 6} => {[7], 1},
               {6, 6} => {[3, 7], 2},
               {0, 4} => {[7], 1},
               {6, 8} => {[2, 3, 7], 3},
               {1, 7} => {[4, 6, 7], 3},
               {4, 2} => {[1], 1},
               {8, 8} => {[1, 2, 3], 3},
               {2, 3} => {[], 0},
               {1, 8} => {[4, 7], 2},
               {8, 2} => {[], 0},
               {3, 4} => {[], 0},
               {7, 5} => {[], 0},
               {2, 1} => {[], 0},
               {7, 0} => {[], 0},
               {7, 8} => {[], 0},
               {4, 7} => {[9], 1},
               {3, 3} => {[2, 7], 2},
               {3, 0} => {[2, 5, 9], 3},
               {7, 7} => {[6, 8, 9], 3},
               {6, 0} => {[], 0},
               {7, 1} => {[3], 1},
               {5, 3} => {[], 0},
               {1, 6} => {[6, 7], 2},
               {4, 1} => {[], 0},
               {5, 5} => {[2], 1},
               {8, 0} => {[2, 3], 2},
               {6, 4} => {[], 0},
               {3, 5} => {[2, 3], 2},
               {1, 0} => {[5, 8], 2},
               {2, 6} => {[], 0},
               {1, 5} => {[4, 5], 2},
               {5, 1} => {[5, 8], 2},
               {2, 5} => {[4], 1},
               {2, 2} => {[4, 7], 2},
               {5, 7} => {[4, 5, 7], 3},
               {0, 2} => {[4, 7], 2},
               {4, 4} => {[], 0},
               {7, 4} => {[3, 9], 2},
               {0, 6} => {[1, 7], 2},
               {6, 7} => {[7], 1},
               {3, 8} => {[], 0},
               {6, 3} => {[2, 4], 2},
               {5, 4} => {[], 0},
               {1, 3} => {[], 0}
             } = candidates
    end
  end
end

// Sudoku Background
// Sudoku is a game played on a 9x9 grid. The goal of the game is to fill all cells of the grid with
// digits from 1 to 9, so that each column, each row, and each of the nine 3x3 sub-grids (also known
// as blocks) contain all of the digits from 1 to 9.
// (More info at: http://en.wikipedia.org/wiki/Sudoku)
//
// Sudoku Solution Validator
// Write a function validSolution/ValidateSolution/valid_solution() that accepts a 2D array
// representing a Sudoku board, and returns true if it is a valid solution, or false otherwise. The
// cells of the sudoku board may also contain 0's, which will represent empty cells. Boards
// containing one or more zeroes are considered to be invalid solutions.
//
// The board is always 9 cells by 9 cells, and every cell only contains integers from 0 to 9.
//

fn main() {
    println!("{}", 11)
}



static STANDARD: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

fn valid_solution(sudoku: &[[u8;9]; 9]) -> bool {
    fn check_array(mut arr: Vec<u8>) -> bool {
        arr.sort();
        arr == STANDARD
    }
    for i in 0..9 {
        if !check_array(sudoku[i].to_vec()) {
            return false;
        }
        if !check_array(sudoku.clone().map(|line| line[i]).to_vec()) {
            return false;
        }
        let line = (i / 3) as usize;
        let col = i % 3;
        let mut square = vec![];
        for m in 0..3 {
            for n in 0..3 {
                square.push(sudoku[line * 3 + m][col * 3 + n]);
            }
        }
        if !check_array(square) {
            return false;
        }
    }
    true
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::valid_solution;

    #[test]
    fn valid_sudoku() {
        let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
            [2, 4, 3, 7, 1, 9, 6, 5, 8],
            [8, 5, 1, 4, 6, 2, 9, 7, 3],
            [4, 8, 6, 9, 7, 5, 3, 1, 2],
            [5, 3, 7, 6, 2, 1, 4, 8, 9],
            [1, 9, 2, 8, 4, 3, 7, 6, 5],
            [6, 1, 8, 3, 5, 4, 2, 9, 7],
            [9, 7, 4, 2, 8, 6, 5, 3, 1],
            [3, 2, 5, 1, 9, 7, 8, 4, 6]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, true, "\nYour result (left) did not match expected result (right).");
    }

    #[test]
    fn invalid_sudoku() {
        let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
            [2, 4, 3, 7, 1, 9, 6, 5, 8],
            [8, 5, 1, 4, 6, 2, 9, 7, 3],
            [4, 8, 6, 9, 7, 5, 3, 1, 2],
            [5, 3, 7, 6, 2, 1, 4, 8, 9],
            [1, 9, 2, 8, 4, 3, 7, 6, 5],
            [6, 1, 8, 3, 5, 4, 2, 9, 7],
            [9, 7, 4, 2, 8, 6, 5, 3, 1],
            [3, 2, 5, 1, 9, 7, 8, 4, 9]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
    }

    #[test]
    fn invalid_with_zeroes() {
        let puzzle = [[3, 1, 5, 8, 4, 7, 6, 2, 9],
            [4, 7, 8, 2, 9, 6, 3, 5, 0],
            [2, 9, 6, 3, 5, 1, 7, 8, 4],
            [7, 4, 2, 9, 6, 8, 5, 1, 3],
            [6, 8, 9, 5, 1, 3, 4, 7, 2],
            [5, 0, 1, 4, 7, 2, 8, 9, 6],
            [1, 2, 4, 6, 8, 5, 9, 3, 7],
            [8, 6, 3, 7, 2, 9, 0, 4, 5],
            [9, 5, 7, 1, 3, 4, 2, 6, 8]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
    }
}

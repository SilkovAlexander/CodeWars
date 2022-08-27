// Write a function that will solve a 9x9 Sudoku puzzle. The function will take one argument
// consisting of the 2D puzzle array, with the value 0 representing an unknown square.
//
// The Sudokus tested against your function will be "easy" (i.e. determinable; there will be no need
// to assume and test possibilities on unknowns) and can be solved with a brute-force approach.
//
// For Sudoku rules, see the Wikipedia article.

fn print(puzzle: &mut [[u8; 9]; 9]) {
    for i in 0..9 {
        println!("{:?}", puzzle[i]);
    }
}

fn main() {
    let mut puzzle = [
        [6, 0, 5, 7, 2, 0, 0, 3, 9],
        [4, 0, 0, 0, 0, 5, 1, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0, 4],
        [0, 9, 0, 0, 3, 0, 7, 0, 6],
        [1, 0, 0, 8, 0, 9, 0, 0, 5],
        [2, 0, 4, 0, 5, 0, 0, 8, 0],
        [8, 0, 0, 0, 0, 3, 0, 2, 0],
        [0, 0, 2, 9, 0, 0, 0, 0, 1],
        [3, 5, 0, 0, 6, 7, 4, 0, 8],
    ];

    sudoku(&mut puzzle);

    println!("{:?}\n{}", puzzle, valid_solution(&puzzle));
}


// Solve the given puzzle in place, no need to return a copy
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let mut try_cnt = 20;
    loop {
        if try_cnt == 0 {
            break;
        }
        try_cnt -= 1;

        for line in 0..9 {
            for col in 0..9 {
                if puzzle[line][col] != 0 {
                    continue;
                }
                let mut possible = vec![1 as u8, 2, 3, 4, 5, 6, 7, 8, 9];
                let remove = |puzzle: &mut [[u8; 9]; 9], possible: &mut Vec<u8>, i_l, i_c| {
                    let et: [u8; 9] = puzzle[i_l];
                    match possible.iter().position(|x| *x == et[i_c]) {
                        Some(index) => { possible.remove(index); },
                        None => {}
                    };
                };
                let start = ((line/3) * 3, (col/3) * 3);
                for i in 0..9 {
                    if i != line {
                        remove(puzzle, &mut possible, i, col);
                    }
                    if i != col {
                        remove(puzzle, &mut possible, line, i);
                    }
                }
                for i in 0..3 {
                    for j in 0..3 {
                        let coord = (start.0 + i, start.1 + j);
                        if coord.0 == line && coord.1 == col {
                            continue;
                        }
                        remove(puzzle, &mut possible, coord.0, coord.1);
                    }
                }

                if possible.len() == 1 {
                    puzzle[line][col] = possible[0];
                }
            }
        }
        if valid_solution(puzzle) {
            break;
        }
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::sudoku;

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }
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


use std::{collections::HashSet, iter::FromIterator};

fn sudoku_1(puzzle: &mut [[u8; 9]; 9]) {
    let s: HashSet<u8> = HashSet::from_iter(1..=9);
    for r in 0..9 {
        for c in 0..9 {
            if puzzle[r][c] == 0 {
                let br = r / 3 * 3;
                let bc = c / 3 * 3;
                let block: HashSet<u8> = HashSet::from_iter(
                    (0..3_usize)
                        .flat_map(|row| (0..3_usize).map(move |col| (row, col)))
                        .map(|(r, c)| puzzle[br + r][bc + c]),
                );
                let row = HashSet::from(puzzle[r]);
                let col = HashSet::from_iter(puzzle.iter().map(|row| row[c]));
                let x = &s - &(&(&row | &col) | &block);
                if x.len() == 1 {
                    puzzle[r][c] = *x.iter().next().unwrap() as u8;
                    sudoku(puzzle)
                }
            }
        }
    }
}
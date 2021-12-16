// Given a Sudoku data structure with size NxN, n > 0 and √n == integer, write a method to validate if it has been filled out correctly.
//
// The data structure is a multi-dimensional Array, i.e:
//
// [
// [7,8,4,  1,5,9,  3,2,6],
// [5,3,9,  6,7,2,  8,4,1],
// [6,1,2,  4,3,8,  7,5,9],
//
// [9,2,8,  7,1,5,  4,6,3],
// [3,5,7,  8,4,6,  1,9,2],
// [4,6,1,  9,2,3,  5,8,7],
//
// [8,7,6,  3,9,4,  2,1,5],
// [2,4,3,  5,6,1,  9,7,8],
// [1,9,5,  2,8,7,  6,3,4]
// ]
// Rules for validation
//
// Data structure dimension: NxN where n > 0 and √n == integer
// Rows may only contain integers: 1..n (n included)
// Columns may only contain integers: 1..n (n included)
// 'Little squares' (3x3 in example above) may also only contain integers: 1..n (n included)

struct Sudoku{
    data: Vec<Vec<u32>>,
}


impl Sudoku{
    fn is_valid(&self) -> bool {
        let n = self.data.len();
        let root = (n as f64).sqrt() as usize;

        for row in 0..n {
            if self.data[row].len() != n {
                return false;
            }
            let mut mask: u128 = (1 << (n+1)) - 2;
            let mut mask2: u128 = (1 << (n+1)) - 2;
            for col in 0..n {
                mask ^= 1 << self.data[row][col];
                mask2 ^= 1 << self.data[col][row];
            }
            if mask != 0 || mask2 != 0 {
                return false;
            }
        }

        for square in 0..n {
            let mut mask: u128 = (1 << (n+1)) - 2;
            for row in 0..root {
                for col in 0..root {
                    mask ^= 1 << self.data[row + root * ((square / root) as usize)][col + root * (square % root)];
                }
            }
            if mask != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let good_sudoku_1 = Sudoku{
        data: vec![
            vec![7,8,4, 1,5,9, 3,2,6],
            vec![5,3,9, 6,7,2, 8,4,1],
            vec![6,1,2, 4,3,8, 7,5,9],

            vec![9,2,8, 7,1,5, 4,6,3],
            vec![3,5,7, 8,4,6, 1,9,2],
            vec![4,6,1, 9,2,3, 5,8,7],

            vec![8,7,6, 3,9,4, 2,1,5],
            vec![2,4,3, 5,6,1, 9,7,8],
            vec![1,9,5, 2,8,7, 6,3,4]
        ]
    };
    let good_sudoku_2 = Sudoku{
        data: vec![
            vec![1, 4,  2, 3],
            vec![3, 2,  4, 1],

            vec![4, 1,  3, 2],
            vec![2, 3,  1, 4],
        ]
    };
    println!("{}", good_sudoku_1.is_valid());
    println!("{}", good_sudoku_2.is_valid());
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku{
        data: vec![
            vec![7,8,4, 1,5,9, 3,2,6],
            vec![5,3,9, 6,7,2, 8,4,1],
            vec![6,1,2, 4,3,8, 7,5,9],

            vec![9,2,8, 7,1,5, 4,6,3],
            vec![3,5,7, 8,4,6, 1,9,2],
            vec![4,6,1, 9,2,3, 5,8,7],

            vec![8,7,6, 3,9,4, 2,1,5],
            vec![2,4,3, 5,6,1, 9,7,8],
            vec![1,9,5, 2,8,7, 6,3,4]
        ]
    };

    let good_sudoku_2 = Sudoku{
        data: vec![
            vec![1, 4,  2, 3],
            vec![3, 2,  4, 1],

            vec![4, 1,  3, 2],
            vec![2, 3,  1, 4],
        ]
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku{
        data: vec![
            vec![1,2,3, 4,5,6, 7,8,9],
            vec![1,2,3, 4,5,6, 7,8,9],
            vec![1,2,3, 4,5,6, 7,8,9],

            vec![1,2,3, 4,5,6, 7,8,9],
            vec![1,2,3, 4,5,6, 7,8,9],
            vec![1,2,3, 4,5,6, 7,8,9],

            vec![1,2,3, 4,5,6, 7,8,9],
            vec![1,2,3, 4,5,6, 7,8,9],
            vec![1,2,3, 4,5,6, 7,8,9],
        ]
    };

    let bad_sudoku_2 = Sudoku{
        data: vec![
            vec![1,2,3,4,5],
            vec![1,2,3,4],
            vec![1,2,3,4],
            vec![1],
        ]
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}
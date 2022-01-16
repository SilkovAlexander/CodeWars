// Given an n x n array, return the array elements arranged from outermost elements to the middle element, traveling clockwise.
//
// array = [[1,2,3],
// [4,5,6],
// [7,8,9]]
// snail(array) #=> [1,2,3,6,9,8,7,4,5]
// For better understanding, please follow the numbers of the next array consecutively:
//
// array = [[1,2,3],
// [8,9,4],
// [7,6,5]]
// snail(array) #=> [1,2,3,4,5,6,7,8,9]
// This image will illustrate things more clearly:
//
//
// NOTE: The idea is not sort the elements from the lowest value to the highest; the idea is to traverse the 2-d array in a clockwise snailshell pattern.
//
// NOTE 2: The 0x0 (empty matrix) is represented as en empty array inside an array [[]].


fn main() {
    let square = &[
        vec![1,2,3],
        vec![8,9,4],
        vec![7,6,5],
    ];
    println!("{:?}", snail(square));
    let square = &[
        vec![1,2,3,4],
        vec![2,3,4,5],
        vec![1,6,5,6],
        vec![0,9,8,7],
    ];
    println!("{:?}", snail(square));
    let square = &[
        vec![1,2,3,4,5],
        vec![6,7,8,9,6],
        vec![5,4,5,0,7],
        vec![4,3,2,1,8],
        vec![3,2,1,0,9],
    ];
    println!("{:?}", snail(square));
}

// pop first in line
// pop last in column
// pop last in line
// pop first in column

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    // n n-1 n-1 n-2
    // to walk around
    let mut res = vec![];
    let mut tmp_matrix = vec![];
    for m in matrix {
        tmp_matrix.push(m.clone());
    }

    while tmp_matrix.len() > 1 {
        let n = tmp_matrix.len();
        // println!("{:?}\n{:?}", tmp_matrix, res);
        for _ in 0..n {
            res.push(tmp_matrix[0][0]);
            tmp_matrix[0].remove(0);
        }
        tmp_matrix.remove(0);
        // println!("{:?}\n{:?}", tmp_matrix, res);
        for i in 0..n - 1 {
            res.push(tmp_matrix[i].pop().unwrap_or(0));
        }
        // println!("{:?}\n{:?}", tmp_matrix, res);
        if tmp_matrix.len() < 2 {
            break;
        }
        for _ in 0..n - 1 {
            res.push(tmp_matrix[n-2].pop().unwrap_or(0));
        }
        tmp_matrix.pop();
        // println!("{:?}\n{:?}", tmp_matrix, res);
        for i in (0..n - 2).rev() {
            res.push(tmp_matrix[i][0]);
            tmp_matrix[i].remove(0);
        }
    }
    if tmp_matrix[0].len() > 0 {
        res.push(tmp_matrix[0][0]);
    }
    res
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];
        let expected = vec![1,2,3,6,9,8,7,4,5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[
            vec![1,2,3],
            vec![8,9,4],
            vec![7,6,5],
        ];
        let expected = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}



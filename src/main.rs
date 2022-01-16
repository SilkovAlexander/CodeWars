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
}

// snail n * n
// n  n-1     n-1  n-2   n-2   n-3

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let n = matrix.len();
    if n <= 1 {
        return matrix[0].clone();
    }
    let mut res = vec![];
    let mut start = (0, 0);
    // 3 - 1,1
    // 5 - 2,2
    // 2 - 1,0
    // 4 - 2,1
    // 6 - 3,2

    let center = match n % 2 {
        1 => ((n - 1) / 2, (n - 1) / 2),
        _ => (n / 2, n / 2 - 1)
    };

    let mut dir = 0;
    for angle in (1..n).rev() {
        for i in 0..(angle) {
            println!("{:?}", start);
            res.push(matrix[start.0][start.1]);
            match dir == 0 {
                true => start.1 += 1,
                false => start.1 -= 1
            };
        }
        println!("{:?}", start);
        res.push(matrix[start.0][start.1]);
        match dir == 0 {
            true => start.0 += 1,
            false => start.0 -= 1
        };
        for i in 0..angle-1 {
            println!("{:?}", start);
            res.push(matrix[start.0][start.1]);
            match dir == 0 {
                true => start.0 += 1,
                false => start.0 -= 1
            };
        }
        println!("{:?}", start);
        res.push(matrix[start.0][start.1]);
        dir = (dir + 1) % 2;
    }

    println!("{:?}", center);
    res.push(matrix[center.0][center.1]);
    res

    // for i in (2..=n).rev() {
    //     for j in 0..i-1 {
    //         match dir {
    //             0 => start.1 += 1,
    //             _ => start.1 -= 1
    //         }
    //         println!("{:?}", start);
    //         res.push(matrix[start.0][start.1]);
    //     }
    //     for j in 0..i-1 {
    //         match dir {
    //             0 => start.0 += 1,
    //             _ => start.0 -= 1
    //         }
    //         println!("{:?}", start);
    //         res.push(matrix[start.0][start.1]);
    //     }
    //     dir = (dir + 1) % 2;
    // }
    // res.push(matrix[1][1]);
    // res
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



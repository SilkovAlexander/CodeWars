// Lyrics...
// Pyramids are amazing! Both in architectural and mathematical sense. If you have a computer, you can mess with pyramids even if you are not in Egypt at the time. For example, let's consider the following problem. Imagine that you have a pyramid built of numbers, like this one here:
//
// /3/
// \7\ 4
// 2 \4\ 6
// 8 5 \9\ 3
// Here comes the task...
// Let's say that the 'slide down' is the maximum sum of consecutive numbers from the top to the bottom of the pyramid. As you can see, the longest 'slide down' is 3 + 7 + 4 + 9 = 23
//
// Your task is to write a function that takes a pyramid representation as argument and returns its' largest 'slide down'. For example:
//
// * With the input `[[3], [7, 4], [2, 4, 6], [8, 5, 9, 3]]`
// * Your function should return `23`.
// By the way...
// My tests include some extraordinarily high pyramids so as you can guess, brute-force method is a bad idea unless you have a few centuries to waste. You must come up with something more clever than that.
//
// (c) This task is a lyrical version of the Problem 18 and/or Problem 67 on ProjectEuler.


fn main() {
    let small = vec![
        vec![3],
        vec![7, 4],
        vec![2, 4, 6],
        vec![8, 5, 9, 3]
    ];
    println!("{:?}", decompose_pyramid(&small));
    println!("{:?}", longest_slide_down(&small));
}

fn decompose_pyramid(pyramid: &[Vec<u16>]) -> (Vec<Vec<u16>>, Vec<Vec<u16>>) {
    let (mut left, mut right) = (vec![], vec![]);
    for (index, row) in pyramid.iter().enumerate() {
        if index == 0 {
            continue
        }
        let mut n_r = row.clone();
        n_r.remove(0);
        right.push(n_r);
        let mut n_l = row.clone();
        n_l.remove(n_l.len() - 1);
        left.push(n_l);
    }
    (left, right)
}

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    if pyramid.len() == 1 {
        return pyramid[0][0];
    } else {

    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let small = vec![
            vec![3],
            vec![7, 4],
            vec![2, 4, 6],
            vec![8, 5, 9, 3]
        ];
        assert_eq!(longest_slide_down(&small), 23, "It should work for small pyramids");
    }

    #[test]
    fn test_medium() {
        let medium = vec![
            vec![75],
            vec![95, 64],
            vec![17, 47, 82],
            vec![18, 35, 87, 10],
            vec![20,  4, 82, 47, 65],
            vec![19,  1, 23, 75,  3, 34],
            vec![88,  2, 77, 73,  7, 63, 67],
            vec![99, 65,  4, 28,  6, 16, 70, 92],
            vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
            vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
            vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
            vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
            vec![63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
            vec![ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23]
        ];
        assert_eq!(longest_slide_down(&medium), 1074, "It should work for medium pyramids");
    }
}
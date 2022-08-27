// Write an algorithm that takes an array and moves all of the zeros to the end, preserving
// the order of the other elements.

// moveZeros([false,1,0,1,2,0,1,3,"a"]) // returns[false,1,1,2,1,3,"a",0,0]


fn main() {
    println!("{}", 11)
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut zeroes_cnt = 0;
    let mut res: Vec<u8> = arr.to_vec().drain(..).filter(|el| { if (el == &0) { zeroes_cnt += 1; false } else { true } }).collect();
    for _ in 0..zeroes_cnt {
        res.push(0);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(actual == expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        dotest(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}

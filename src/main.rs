/*
For a given list [x1, x2, x3, ..., xn] compute the last (decimal) digit of
x1 ^ (x2 ^ (x3 ^ (... ^ xn))).

E. g., with the input [3, 4, 2], your code should return 1 because 3 ^ (4 ^ 2) = 3 ^ 16 = 43046721.

Beware: powers grow incredibly fast. For example, 9 ^ (9 ^ 9) has more than 369 millions of digits.
lastDigit has to deal with such numbers efficiently.

Corner cases: we assume that 0 ^ 0 = 1 and that lastDigit of an empty list equals to 1.

0 - 0
1 - 1
2 - 2 4 8 6
3 - 3 9 7 1
4 - 4 6
5 - 5
6 - 6
7 - 7 9 3 1
8 - 8 4 2 6
9 - 9 1

Due to the previous table we need to know only the Power % 4.

*/

fn main() {
    println!("{} 6", last_digit(&vec![2, 2, 101, 2]));
    println!("{} 1", last_digit(&vec![3, 4, 5]));
    println!("{} 4", last_digit(&vec![4, 3, 6]));
    println!("{} 1", last_digit(&vec![7, 6, 21]));

}

fn last_digit(list: &[u64]) -> u64 {
    if list.is_empty() {
        return 0;
    }
    if list.len() == 1 {
        return list[0];
    }
    let mut pow = match list.last().unwrap() % 4 {
        0 => 4,
        val => val
    };
    for i in (0..list.len()-1).rev() {
        pow = list[i].pow(pow as u32) % (if i != 0 { 4 } else {10});
    }
    pow

}

#[cfg(test)]
mod tests {
    use super::last_digit;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(v: &[u64], expected: u64) {
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (a, b) in  [
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![2, 2, 101, 2], 6),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6)
        ]  {
            dotest(&a, b);
        }
    }
}

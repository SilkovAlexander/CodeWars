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
But here we need a stairs of powers and we need 2 digits

0 - 0
1 - 1
2 - X2 04 08 16 32 64 28 56 12 24 48 96 92 84 68 36 72 44 88 76      - 20
3 - 03 09 27 81 43 29 87 64 83 49 47 41 23 69 07 21 63 89 67 01      - 20
4 - 04 16 64 56 24 96 84 36 44 76                                    - 10
5 - X5 (25)
6 - X6 36 16 96 76                                                   - 5
7 - 07 49 43 01                                                      - 4
8 - 08 64 12 96 68 44 52 16 28 24 92 36 88 04 32 56 48 84 72 76      - 20
9 - 09 81 29 61 49 41 69 21 89 01                                    - 10
*/

fn main() {
    // println!("{} 6", last_digit(&vec![2, 2, 101, 2]));
    // println!("{} 1", last_digit(&vec![3, 4, 5]));
    // println!("{} 4", last_digit(&vec![4, 3, 6]));
    // println!("{} 1", last_digit(&vec![7, 6, 21]));
    // println!("{} 1", last_digit(&vec![]));
    // println!("{} 0", last_digit(&vec![0]));
    // println!("{} 1", last_digit(&vec![0, 0]));
    println!("{} 0", last_digit(&vec![937640, 767456, 981242]));
}

fn last_digit(list: &[u64]) -> u64 {
    if list.is_empty() {
        return 1;
    }
    if list.len() == 1 {
        return list[0];
    }
    let mod_ = |num: u128|
        if num < 20 {
            num
        } else {
            match num % 20 {
            0 => 20,
            1 => 21,
            val => val
    }};
    let mut pow = mod_(list.last().unwrap().to_owned() as u128);
    for i in (0..list.len()-1).rev() {
        let num = mod_(list[i] as u128);
        pow = mod_(num.pow(pow as u32));
    }
    pow as u64 % 10

}

#[cfg(test)]
mod tests {
    use super::last_digit;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(v: &[u64], expected: u64) {
        let vv = v.to_owned().clone();
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {vv:?}")
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


fn clever_last_digit(lst: &[u64]) -> u64 {
    let f = |x, y| std::cmp::min(x % y + y, x);
    lst.iter().rev().fold(1, |v, &n| f(n, 20).pow(f(v, 4) as u32)) % 10
}
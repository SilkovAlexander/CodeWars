// Create a function that takes a positive integer and returns the next bigger number that can be formed by rearranging its digits. For example:
//
// 12 ==> 21
// 513 ==> 531
// 2017 ==> 2071
// nextBigger(num: 12)   // returns 21
// nextBigger(num: 513)  // returns 531
// nextBigger(num: 2017) // returns 2071
// If the digits can't be rearranged to form a bigger number, return -1 (or nil in Swift):
//
// 9 ==> -1
// 111 ==> -1
// 531 ==> -1
// nextBigger(num: 9)   // returns nil
// nextBigger(num: 111) // returns nil
// nextBigger(num: 531) // returns nil

fn concat(digits: Vec<u64>) -> u64 {
    let mut res = 0;
    for d in digits {
        res = res * 10 + d;
    }
    res
}

fn next_bigger_number(n: i64) -> i64 {
    let mut digits: Vec<u64> = [].to_vec();
    let init = n.clone() as u64;
    let mut n = n;
    while n != 0 {
        digits.insert(0, n as u64 % 10);
        n = n  / 10;
    }
    // println!("{:?}", digits);
    let mut max: Option<u64> = None;
    for i in (1..digits.len()).rev() {
        for j in 0..i {
            let mut digits2 = digits.clone();
            let tmp = digits2[i];
            digits2[i] = digits2[j];
            digits2[j] = tmp;
            let tmp = concat(digits2);
            if tmp > init && (max.is_none() || tmp < max.unwrap()) {
                println!("{}", tmp);
                max = Some(tmp);
            }
        }
    }
    // println!("{:?}", digits);

    if max.is_some() {
        return max.unwrap() as i64;
    }
    return -1;
}

fn main() {
    println!("{:?}", next_bigger_number(1234567890));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
        assert_eq!(1234567908, next_bigger_number(1234567890));
    }
}

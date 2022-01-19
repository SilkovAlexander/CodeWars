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


fn next_bigger_number(n: i64) -> i64 {
    // represent the number as a vector of digits
    let digits = n.to_string()
        .chars()
        .into_iter()
        .map(|c| i64::from_str_radix(&c.to_string(), 10).unwrap())
        .collect::<Vec<i64>>();

    // iterate through the digits from the right side
    for i in (0..digits.len() - 1).rev() {
        // if digit to the left is greater than right start procedure
        if digits[i] < digits[i + 1] {
            // split vector
            let (left, right) = digits.split_at(i);
            let mut right = right.to_vec();

            // search for the minimal digit that is to the right from the chosen one
            // and greater than it
            let mut min = 1;
            for j in 2..right.len() {
                if right[j] > right[0] && right[j] < right[min] {
                    min = j;
                }
            }
            // put it first
            let first = right.remove(min);
            // sort the latter
            right.sort();
            right.insert(0, first);
            // assemble the number
            let mut res = left.to_vec();
            res.append(&mut right);
            return i64::from_str_radix(&res.iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(""), 10).unwrap();
        }
    }

    return -1;
}

fn main() {
    println!("{:?}", next_bigger_number(1234567890));
    println!("{:?}", next_bigger_number(988877776665544321));
    println!("{:?}", next_bigger_number(7665544321));
    println!("{:?}", next_bigger_number(12345));
    // for i in 1..1000000 {
    //     let _ = next_bigger_number(i);
    // }
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

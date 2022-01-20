// Write a function that takes a positive integer and returns the next smaller positive integer containing the same digits.
//
// For example:
//
// next_smaller(21) == Some(12)
// next_smaller(531) == Some(513)
// next_smaller(2071) == Some(2017)
// Return -1 (for Haskell: return Nothing, for Rust: return None), when there is no smaller number that contains the same digits. Also return -1 when the next smaller number with the same digits would require the leading digit to be zero.
//
// next_smaller(9) == None
// next_smaller(135) == None
// next_smaller(1027) == None  // 0721 is out since we don't write numbers with leading zeros
// some tests will include very large numbers.
// test data only employs positive integers.
// The function you write for this challenge is the inverse of this kata: "Next bigger number with the same digits."

fn next_smaller_number(n: u64) -> Option<u64> {
    // represent the number as a vector of digits
    let digits = n.to_string()
        .chars()
        .into_iter()
        .map(|c| u64::from_str_radix(&c.to_string(), 10).unwrap())
        .collect::<Vec<u64>>();

    // iterate through the digits from the right side
    for i in (0..digits.len() - 1).rev() {
        // if digit to the left is less than right start procedure
        if digits[i] > digits[i + 1] {
            // split vector
            let (left, right) = digits.split_at(i);
            let mut right = right.to_vec();

            // search for the maximal digit that is to the right from the chosen one
            // and less than it
            let mut max = 1;
            for j in 2..right.len() {
                if right[j] < right[0] && right[j] > right[max] {
                    max = j;
                }
            }
            // put it first
            let first = right.remove(max);
            if first == 0 && i ==0 {
                continue;
            }
            // sort the latter
            right.sort_by(|a, b| b.cmp(a));
            right.insert(0, first);
            // assemble the number
            let mut res = left.to_vec();
            res.append(&mut right);
            return Some(u64::from_str_radix(&res.iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(""), 10).unwrap());
        }
    }

    None
}

fn main() {
    println!("{:?}", next_smaller_number(21));
    println!("{:?}", next_smaller_number(1234567908));
    println!("{:?}", next_smaller_number(441));
    println!("{:?}", next_smaller_number(12345));
    println!("{:?}", next_smaller_number(1027));
    println!("{:?}", next_smaller_number(1000));
    println!("{:?}", next_smaller_number(1207));
    println!("{:?}", next_smaller_number(4372236780989090620));
    // for i in 1..1000000 {
    //     let _ = next_bigger_number(i);
    // }
}


// tests::extended
// Log
// 513
// 351
// 315
// 153
// 135
// 2071
// 1207
// assertion failed: `(left == right)`
// left: `Some(1072)`,
// right: `None`
// tests::random
// Log
// 4989908415404129929
// 13766676098178774043
// 2585089382346300873
// 12405809614208060748
// 15298778596191880198
// 1375595996145508665
// 9402965508040124441
// 4372236780989090620
// assertion failed: `(left == right)`
// left: `Some(4372236780989090602)`,
// right: `Some(4372236780989090260)`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
        assert_eq!(Some(1072), next_smaller_number(1207));
        assert_eq!(Some(4372236780989090602), next_smaller_number(4372236780989090620));
    }
}

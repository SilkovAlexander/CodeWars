// Digital root is the recursive sum of all the digits in a number.
//
// Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer.
//
// Examples
// 16  -->  1 + 6 = 7
// 942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
// 132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
// 493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2


fn digital_root(n: i64) -> i64 {
    if n < 10 {
        return n;
    }
    digital_root(
        n.to_string()
        .chars()
        .into_iter()
        .map(|c| i64::from_str_radix(&c.to_string(), 10).unwrap_or(0))
        .collect::<Vec<i64>>()
        .iter()
        .sum()
    )
}

fn main() {
    println!("{:?}", digital_root(16));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }
}

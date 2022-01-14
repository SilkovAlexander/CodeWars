// Write a function that takes an integer as input, and returns the number of bits that are equal to
// one in the binary representation of that number. You can guarantee that input is non-negative.
//
// Example: The binary representation of 1234 is 10011010010, so the function should return 5 in
// this case


fn main() {
    println!("{:?}", count_bits(10));
}

fn count_bits(n: i64) -> u32 {
    let mut n = n;
    let mut res = 0;
    while n != 0 {
        res += (n % 2) as u32;
        n = n >> 1;
    }
    res
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}
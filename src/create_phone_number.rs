// Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.
//
// Example
// create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
// The returned format must be correct in order to complete this challenge.
// Don't forget the space after the closing parentheses!


fn create_phone_number(numbers: &[u8]) -> String {
    let joiner = |n:&[u8]| {
        n.iter().map(|d| format!("{}",d)).collect::<Vec<String>>().join("")
    };
    format!("({}) {}-{}", joiner(&numbers[0..=2]), joiner(&numbers[3..=5]), joiner(&numbers[6..=9]))
}


fn main() {
    println!("{:?}", create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
}

#[test]
fn returns_expected() {
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
    assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
    assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
}
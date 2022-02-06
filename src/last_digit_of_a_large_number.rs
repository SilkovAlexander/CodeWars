// Define a function that takes in two non-negative integers aaa and bbb and returns the last decimal digit of aba^ba
// b
// . Note that aaa and bbb may be very large!
//
// For example, the last decimal digit of 979^79
// 7
// is 999, since 97=47829699^7 = 47829699
// 7
// =4782969. The last decimal digit of (2200)2300({2^{200}})^{2^{300}}(2
// 200
// )
// 2
// 300
//
// , which has over 109210^{92}10
// 92
// decimal digits, is 666. Also, please take 000^00
// 0
// to be 111.
//
// You may assume that the input will always be valid.
//
// Examples
// last_digit("4", "1")            // returns 4
// last_digit("4", "2")            // returns 6
// last_digit("9", "7")            // returns 9
// last_digit("10","10000000000")  // returns 0
// Remarks
// JavaScript, C++, R, PureScript
// Since these languages don't have native arbitrarily large integers, your arguments are going to be strings representing non-negative integers instead.

/*
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
 */



fn last_digit(str1: &str, str2: &str) -> i32 {
    if str2 == "0" {
        return 1;
    }
    let last_digit = str1.chars().last().unwrap().to_digit(10).unwrap() as i32;
    let range = if str2.len() > 2 {
        str2.len() - 2..str2.len()
    } else {
        0..str2.len()
    };
    let last_power = u32::from_str_radix(&str2[range], 10).unwrap();
    // println!("{} {}", last_digit, last_power);
    let last_power = if last_power % 4 == 0 { 4 } else { last_power  % 4 };
    (last_digit.pow(last_power)) % 10
}


fn main() {
    assert_eq!(last_digit("0", "0"), 0);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10","10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}

#[test]
fn returns_expected() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10","10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
}
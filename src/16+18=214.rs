// For this kata you will have to forget how to add two numbers.
//
// It can be best explained using the following meme:
//
// Dayane Rivas adding up a sum while competing in the Guatemalan television show "Combate" in May 2016
//
// In simple terms, our method does not like the principle of carrying over numbers and just writes down every number it calculates :-)
//
// You may assume both integers are positive integers.
//
// Examples
// 16 + 18 = 2 14


fn main() {
    println!("{:?}", add(16, 18));
}

fn add(num1: u32, num2: u32) -> u64 {
    let convert = |num: u32| {
        num.to_string()
            .chars()
            .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap_or(0))
            .rev()
            .collect::<Vec<u32>>()
    };
    let num1 = convert(num1);
    let num2 = convert(num2);
    let (mut long, short) = if num1.len() > num2.len() {
        (num1, num2)
    } else {
        (num2, num1)
    };
    for i in 0..long.len() {
        long[i] += short.get(i).unwrap_or(&0);
    }
    let res = long.iter().rev()
        .map(|dig| dig.to_string().chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    u64::from_str_radix(&res, 10).unwrap_or(0)
}

#[test]
fn test_real_add() {
    assert_eq!(add(2, 11), 13);
    assert_eq!(add(0, 1), 1);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_silly_add() {
    assert_eq!(add(16, 18), 214);
    assert_eq!(add(26, 39), 515);
    assert_eq!(add(122, 81), 1103);
}
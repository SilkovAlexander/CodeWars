// Coding decimal numbers with factorials is a way of writing out numbers in a base system that depends on factorials, rather than powers of numbers.
//
// In this system, the last digit is always 0 and is in base 0!. The digit before that is either 0 or 1 and is in base 1!. The digit before that is either 0, 1, or 2 and is in base 2!, etc. More generally, the nth-to-last digit is always 0, 1, 2, ..., n and is in base n!.
//
// Read more about it at: http://en.wikipedia.org/wiki/Factorial_number_system
//
// Example
// The decimal number 463 is encoded as "341010", because:
//
// 463 = 3×5! + 4×4! + 1×3! + 0×2! + 1×1! + 0×0!
//
// If we are limited to digits 0..9, the biggest number we can encode is 10!-1 (= 3628799). So we extend 0..9 with letters A..Z. With these 36 digits we can now encode numbers up to 36!-1 (= 3.72 × 1041)
//
// Task
// We will need two functions. The first one will receive a decimal number and return a string with the factorial representation.
//
// The second one will receive a string with a factorial representation and produce the decimal representation.
//
// Given numbers will always be positive.


fn main() {
    println!("{:?}", dec2_fact_string(463));
    println!("{:?}", dec2_fact_string(36288000));
    println!("{:?}", fact_string_2dec("341010".to_string()));
}

fn fact(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fact(n - 1) * n
    }
}

fn dec2_fact_string(nb: u64) -> String {
    let mut facts = vec![];
    let mut max = 0;
    loop {
        let f = fact(max);
        if f > nb {
            break
        }
        facts.push(f);
        max += 1;
    }
    facts.reverse();
    let mut nb = nb;
    let mut digits = String::new();
    for f in facts {
        let dig = nb / f;
        nb = nb % f;
        let dig = if dig < 10 {
            char::from_digit(dig as u32, 10).unwrap_or('0')
        } else {
            char::from_u32((('A' as u64) + dig - 10) as u32).unwrap_or('0')
        };
        digits.push(dig);
    }
    digits
}

fn fact_string_2dec(s: String) -> u64 {
    let mut res: u64 = 0;
    let mut facts = vec![];
    for i in 0..s.len() {
        facts.push(fact(i as u64));
    }
    for c in s.chars().collect::<Vec<char>>() {
        let dig = match c.to_digit(10) {
            Some(dig) => dig,
            None => c as u32 + 10 - ('A' as u32)
        };
        res += facts.last().unwrap() * &(dig as u64);
        facts.pop();
    }
    res
}

fn testing1(nb: u64, exp: &str) -> () {
    assert_eq!(&dec2_fact_string(nb), exp)
}

fn testing2(s: &str, exp: u64) -> () {
    assert_eq!(fact_string_2dec(s.to_string()), exp)
}

#[test]
fn basics_dec2_fact_string() {

    testing1(2982, "4041000");
    testing1(463, "341010");

}
#[test]
fn basics_fact_string_2dec() {

    testing2("4041000", 2982);
    testing2("341010", 463);

}

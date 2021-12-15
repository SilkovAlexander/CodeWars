// Write a function that takes a string of braces, and determines if the order of the braces is valid. It should return true if the string is valid, and false if it's invalid.
//
// This Kata is similar to the Valid Parentheses Kata, but introduces new characters: brackets [], and curly braces {}. Thanks to @arnedag for the idea!
//
// All input strings will be nonempty, and will only consist of parentheses, brackets and curly braces: ()[]{}.
//
// What is considered Valid?
// A string of braces is considered valid if all braces are matched with the correct brace.
//
// Examples
// "(){}[]"   =>  True
// "([{}])"   =>  True
// "(}"       =>  False
// "[(])"     =>  False
// "[({})](]" =>  False


fn valid_braces(s: &str) -> bool {
    fn is_open(c: char) -> bool {
        c == '(' || c == '{' || c == '['
    }
    fn is_close(c: char) -> bool {
        c == ')' || c == '}' || c == ']'
    }
    fn map_open(c: char) -> char {
        match c {
            '(' => ')',
            '{' => '}',
            '[' => ']',
            _ => '\0'
        }
    }
    let mut chars: Vec<char> = Vec::new();
    for c in s.chars() {
        if is_open(c.clone()) {
            chars.push(map_open(c.clone()));
        } else if is_close(c.clone()) && !chars.is_empty() {
            if chars.last().unwrap().to_owned() != c {
                return false;
            }
            chars.pop();
        } else {
            return false;
        }
    }
    chars.is_empty()
}

fn main() {
    println!("{}", valid_braces("(((({{"));
    println!("{}", valid_braces("()[]{}({[]})"));
    println!("{}", valid_braces(")"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
        expect_false("(((({{");
    }

    fn expect_true(s: &str) {
        assert!(valid_braces(s), "Expected {s:?} to be valid. Got false", s=s);
    }

    fn expect_false(s: &str) {
        assert!(!valid_braces(s), "Expected {s:?} to be invalid. Got true", s=s);
    }
}
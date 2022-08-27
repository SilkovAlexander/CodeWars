// Write a function that takes a string of parentheses, and determines if the order of the
// parentheses is valid. The function should return true if the string is valid, and false if it's
// invalid.
//
// Examples
// "()"              =>  true
// ")(()))"          =>  false
// "("               =>  false
// "(())((()())())"  =>  true
// Constraints
// 0 <= input.length <= 100
//
// Along with opening (() and closing ()) parenthesis, input may contain any valid ASCII characters.
// Furthermore, the input string may be empty and/or not contain any parentheses at all. Do not
// treat other forms of brackets as parentheses (e.g. [], {}, <>).
//


fn main() {
    println!("{}", 11)
}

fn valid_parentheses(s: &str) -> bool {
    let mut cnt: i8 = 0;
    for c in s.chars() {
        if c == '(' {
            cnt += 1;
        } else if c == ')' {
            if cnt == 0 {
                return false;
            }
            cnt -= 1;
        }
    }
    cnt == 0
}

#[cfg(test)]
mod tests {
    use super::valid_parentheses;

    #[test]
    fn sample_tests() {
        do_test("()", true);
        do_test("())", false);
        do_test("", true);
        do_test("(}{)", true);
    }

    fn do_test(s: &str, exp: bool) {
        let actual = valid_parentheses(s);
        assert_eq!(
            actual,
            exp,
            "\nFor the input \"{}\", your result ({}) did not match the expected output ({})",
            s, actual, exp
        );
    }
}
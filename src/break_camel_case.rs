// Complete the solution so that the function will break up camel casing, using a space between words.
//
// Example
// "camelCasing"  =>  "camel Casing"
// "identifier"   =>  "identifier"
// ""             =>  ""

fn solution(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c >= 'A' && c <= 'Z' && !res.is_empty() {
            res += " ";
        }
        res.push(c);
    }
    res
}

fn main() {
    println!("{}", solution("camelCasing"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}

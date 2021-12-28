// ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.
//
// Create a function that takes a string and returns the string ciphered with Rot13. If there are numbers or special characters included in the string, they should be returned as they are. Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".
//



fn rot13(message: &str) -> String {
   message.chars().map( |c|
        if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
            if (c >= 'a' && c >= 'n') || (c <= 'Z' && c>= 'N') {
                (c as u8 - 13) as char
            } else {
                (c as u8 + 13) as char
            }
        } else {
            c
        }
    ).collect()
}


fn main() {
    println!("{:?}", rot13("Test"));
    println!("{:?}", rot13("ANan"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}

// Create two functions to encode and then decode a string using the Rail Fence Cipher. This cipher
// is used to encode a string by placing each character successively in a diagonal along a set of
// "rails". First start off moving diagonally and down. When you reach the bottom, reverse direction
// and move diagonally and up until you reach the top rail. Continue until you reach the end of the
// string. Each "rail" is then read left to right to derive the encoded string.
//
// For example, the string "WEAREDISCOVEREDFLEEATONCE" could be represented in a three rail system
// as follows:
//
// W       E       C       R       L       T       E  7
//   E   R   D   S   O   E   E   F   E   A   O   C    12
//     A       I       V       D       E       N      6
// The encoded string would be:
//
// WECRLTEERDSOEEFEAOCAIVDEN
//
// Write a function/method that takes 2 arguments, a string and the number of rails, and returns the
// ENCODED string.
//
// Write a second function/method that takes 2 arguments, an encoded string and the number of rails,
// and returns the DECODED string.
//
// For both encoding and decoding, assume number of rails >= 2 and that passing an empty string will
// return an empty string.
//
// Note that the example above excludes the punctuation and spaces just for simplicity. There are,
// however, tests that include punctuation. Don't filter out punctuation as they are a part of the
// string.

/*
rail  symbols in `column`
  0     1
  1     2
  2     2
  3     2

  r-2   2
  r-1   1

number of symbols in column = 1 + 1 + 2 * (r-2) = 2r - 2


1
 2 8
 3 7
 4 6
  5

11112828282823737373737464646465555

1   5
 2 4 6 8
  3   7

 */



fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    if text.is_empty() {
        return text.to_owned();
    }
    match num_rails {
        0 | 1 => text.to_owned(),
        _ => {
            let mut chunks = vec![];
            let chunk_len = 2 * num_rails - 2;
            let mut last = text;
            while last.len() > chunk_len {
                let (first, last_) = last.split_at(chunk_len);
                chunks.push(first.chars().collect::<Vec<char>>());
                last = last_;
            }
            chunks.push(last.chars().collect());
            let mut res= String::new();
            for r in 0..num_rails {
                for chunk in chunks.iter() {
                    res.push(chunk.get(r).unwrap_or(&'\0').to_owned());
                    if r != 0 && r != num_rails - 1 {
                        res.push(chunk.get(chunk_len - r).unwrap_or(&'\0').to_owned());
                    }
                }
            }
            res.split('\0').collect::<Vec<&str>>().join("")
        }
    }
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    if text.is_empty() {
        return text.to_owned();
    }
    match num_rails {
        0 | 1 => text.to_owned(),
        _ => {
            let chunk_len = 2 * num_rails - 2;
            let (chunk_cnt, last_size) = (text.len() / chunk_len, text.len() % chunk_len);
            let mut chunks = vec![];

            let mut last = text;
            while last.len() > chunk_len {
                let (first, last_) = last.split_at(chunk_len);
                chunks.push(first.chars().collect::<Vec<char>>());
                last = last_;
            }
            chunks.push(last.chars().collect());
            println!("{:?}", chunks);
            "".to_owned()
        }
    }
}

fn main() {
    println!("{:?}", encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3));
    println!("{:?}", decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
        // assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
        assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
        // assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
        assert_eq!(encode_rail_fence_cipher("12345678", 3), "15246837");
        assert_eq!(encode_rail_fence_cipher("1234567", 3), "1524637");
        assert_eq!(encode_rail_fence_cipher("123456", 3), "152463");
        assert_eq!(encode_rail_fence_cipher("12345", 3), "15243");
    }
}

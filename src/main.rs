// Consider a "word" as any sequence of capital letters A-Z (not limited to just "dictionary
// words"). For any word with at least two different letters, there are other words composed of the
// same letters but in a different order (for instance, STATIONARILY/ANTIROYALIST, which happen to
// both be dictionary words; for our purposes "AAIILNORSTTY" is also a "word" composed of the same
// letters as these two).
//
// We can then assign a number to every word, based on where it falls in an alphabetically sorted
// list of all words made up of the same group of letters. One way to do this would be to generate
// the entire list of words and find the desired one, but this would be slow if the word is long.
//
// Given a word, return its number. Your function should be able to accept any word 25 letters or
// less in length (possibly with some letters repeated), and take no more than 500 milliseconds to
// run. To compare, when the solution code runs the 27 test cases in JS, it takes 101ms.
//
// For very large words, you'll run into number precision issues in JS (if the word's position is
// greater than 2^53). For the JS tests with large positions, there's some leeway (.000000001%).
// If you feel like you're getting it right for the smaller ranks, and only failing by rounding on
// the larger, submit a couple more times and see if it takes.
//
// Python, Java and Haskell have arbitrary integer precision, so you must be precise in those
// languages (unless someone corrects me).
//
// C# is using a long, which may not have the best precision, but the tests are locked so we can't
// change it.
//
// Sample words, with their rank:
// AABB - 1  ABAB - 2
// ABAB = 2
// AAAB = 1
// BAAA = 4
// QUESTION = 24572
// BEEEKKOOPR = 1
// 1222334456
// BOOKKEEPER = 10743
// 1443322526

/*
1234    1223
1243    1232
1324    1322
1342    2123
1423    2132
1432    2213
2134    2231
2143    2312
2314    2321  2321
2341    3122
2413    3212
2431    3221
3124
3142
3214
3241
3412
3421
4123
4132
4213
4231
4312
4321

total cnt = n!

22333
23233
23323
23332
32233
32323
32332
33223
33232
33322

 */

use std::collections::{HashMap};

fn main() {
    println!("{}", get_number_of_permutations(vec![1,2,3,4]));
    println!("{}", get_number_of_permutations(vec![1,2,3,3]));
    println!("{}", list_position("BOOKKEEPER"));
}

fn fact(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => n * fact(n - 1)
    }
}

fn get_number_of_permutations(input: Vec<u8>) -> u128 {
    let mut counts: HashMap<u8, u8> = HashMap::new();
    for el in input.clone() {
        *counts.entry(el).or_insert(0) += 1;
    }
    let counts: HashMap<&u8, &u8> = counts.iter().filter(|(_, v)| v != &&1_u8).collect();
    let mut res = fact(input.len() as u128);
    for (_, v) in counts {
        res = res / fact(v.to_owned() as u128);
    }
    res
}

fn list_position(word: &str) -> u128 {
    let mut chars = word.chars().map(|c| c as u8 - 'A' as u8).collect::<Vec<u8>>();
    let mut sorted = chars.clone();
    sorted.sort();
    while chars.len() != 0 {
        if chars[0] == sorted[0] {
            chars.remove(0);
            sorted.remove(0);
        } else {
            break;
        }
    }
    let mut res = 1;
    while chars != sorted && !chars.is_empty() {
        let cur_char = chars.remove(0);
        let index = sorted.iter().position(|x| *x == cur_char).unwrap();
        let mut i = 0;
        while i < index {
            let mut sort_clone = sorted.clone();
            let prev = sort_clone.remove(i);
            res += get_number_of_permutations(sort_clone.clone()) as u128;
            while sort_clone[i] == prev {
                i += 1;
            }
            i += 1;
        }
        sorted.remove(index);
    }
    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::list_position;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn sample_tests() {
        let test_data = [
            (                  "A", 1),
            (               "ABAB", 2),
            (               "AAAB", 1),
            (               "BAAA", 4),
            (               "YMYM", 5),
            (           "QUESTION", 24572),
            (         "BOOKKEEPER", 10743),
            ("IMMUNOELECTROPHORETICALLY", 718393983731145698173),
        ];
        for (word, expected) in test_data {
            assert_eq!(list_position(word),
                       expected,
                       "\nYour result (left) did not match the expected output (right) for the input: \"{word}\"");
        }

    }
}
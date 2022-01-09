// What is an anagram? Well, two words are anagrams of each other if they both contain the same letters. For example:
//
// 'abba' & 'baab' == true
//
// 'abba' & 'bbaa' == true
//
// 'abba' & 'abbba' == false
//
// 'abba' & 'abca' == false
// Write a function that will find all the anagrams of a word from a list. You will be given two inputs a word and an array with words. You should return an array of all the anagrams or an empty array if there are none. For example:
//
// anagrams('abba', ['aabb', 'abcd', 'bbaa', 'dada']) => ['aabb', 'bbaa']
//
// anagrams('racer', ['crazer', 'carer', 'racar', 'caers', 'racer']) => ['carer', 'racer']
//
// anagrams('laser', ['lazing', 'lazy',  'lacer']) => []
// Note for Go
// For Go: Empty string slice is expected when there are no anagrams found.


fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut res = vec![];
    let mut word = word.chars().collect::<Vec<char>>();

    word.sort();
    words.iter().for_each(|w| {
        let mut chars = w.chars().collect::<Vec<char>>();
        chars.sort();
        if chars == word {
            res.push(w.to_string());
        }
    });
    res
}


fn main() {
    println!("{:?}", anagrams(
        "racer",
        &["crazer".to_string(), "carer".to_string()]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);

        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}

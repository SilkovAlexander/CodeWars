/*
Alright, detective, one of our colleagues successfully observed our target person, Robby the robber.
 We followed him to a secret warehouse, where we assume to find all the stolen stuff. The door to
  this warehouse is secured by an electronic combination lock. Unfortunately our spy isn't sure
   about the PIN he saw, when Robby entered it.

The keypad has the following layout:

┌───┬───┬───┐
│ 1 │ 2 │ 3 │
├───┼───┼───┤
│ 4 │ 5 │ 6 │
├───┼───┼───┤
│ 7 │ 8 │ 9 │
└───┼───┼───┘
    │ 0 │
    └───┘
He noted the PIN 1357, but he also said, it is possible that each of the digits he saw could
 actually be another adjacent digit (horizontally or vertically, but not diagonally). E.g. instead
  of the 1 it could also be the 2 or 4. And instead of the 5 it could also be the 2, 4, 6 or 8.

He also mentioned, he knows this kind of locks. You can enter an unlimited amount of wrong PINs,
 they never finally lock the system or sound the alarm. That's why we can try out all possible (*)
  variations.

* possible in sense of: the observed PIN itself and all variations considering the adjacent digits

Can you help us to find all those variations? It would be nice to have a function, that returns an
array (or a list in Java/Kotlin and C#) of all variations for an observed PIN with a length of 1 to
 8 digits. We could name the function getPINs (get_pins in python, GetPINs in C#). But please note
  that all PINs, the observed one and also the results, must be strings, because of potentially
   leading '0's. We already prepared some test cases for you.

Detective, we are counting on you!
*/

use std::collections::HashMap;

fn main() {
    println!("{:?}", get_pins("0"));
    println!("{:?}", get_pins("1345"));
}

fn get_pins(observed: &str) -> Vec<String> {
    let mut neighbours = HashMap::new();
    neighbours.insert('0', vec!['0', '8']);
    neighbours.insert('1', vec!['1', '2', '4']);
    neighbours.insert('2', vec!['2', '1', '3', '5']);
    neighbours.insert('3', vec!['3', '2', '6']);
    neighbours.insert('4', vec!['4', '7', '1', '5']);
    neighbours.insert('5', vec!['5', '8', '2', '4', '6']);
    neighbours.insert('6', vec!['6', '9', '3', '5']);
    neighbours.insert('7', vec!['7', '8', '4']);
    neighbours.insert('8', vec!['8', '5', '7', '9', '0']);
    neighbours.insert('9', vec!['9', '8', '6']);
    let mut result = vec![];
    for c in observed.chars() {
        if result.is_empty() {
            result.append(&mut neighbours.get(&c).unwrap().clone().iter().map(|c| c.to_string()).collect());
        } else {
            let mut n_result = vec![];
            for beg in result {
                for end in neighbours.get(&c).unwrap() {
                    let mut n_str = beg.clone();
                    n_str.push(end.to_owned());
                    n_result.push(n_str);
                }
            }
            result = n_result;
        }

    }
    result
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;


    #[test]
    fn sample_tests() {
        assert_eq!(get_pins("8").iter().sorted().collect::<Vec<&String>>(),
                   vec!["0", "5", "7", "8", "9"]);
        assert_eq!(get_pins("11").iter().sorted().collect::<Vec<&String>>(),
                   vec!["11", "12", "14",  "21", "22", "24",  "41", "42", "44"]);
        assert_eq!(get_pins("369").iter().sorted().collect::<Vec<&String>>(),
                   vec!["236", "238", "239",  "256", "258", "259",  "266", "268", "269",  "296", "298", "299",
                        "336", "338", "339",  "356", "358", "359",  "366", "368", "369",  "396", "398", "399",
                        "636", "638", "639",  "656", "658", "659",  "666", "668", "669",  "696", "698", "699"]);
    }
}

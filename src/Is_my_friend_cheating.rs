// A friend of mine takes the sequence of all numbers from 1 to n (where n > 0).
// Within that sequence, he chooses two numbers, a and b.
// He says that the product of a and b should be equal to the sum of all numbers in the sequence, excluding a and b.
// Given a number n, could you tell me the numbers he excluded from the sequence?
// The function takes the parameter: n (n is always strictly greater than 0) and returns an array or a string (depending on the language) of the form:
//
// [(a, b), ...] or [[a, b], ...] or {{a, b}, ...} or or [{a, b}, ...]
// with all (a, b) which are the possible removed numbers in the sequence 1 to n.
//
// [(a, b), ...] or [[a, b], ...] or {{a, b}, ...} or ... will be sorted in increasing order of the "a".
//
// It happens that there are several possible (a, b). The function returns an empty array (or an empty string) if no possible numbers are found which will prove that my friend has not told the truth! (Go: in this case return nil).
//
// Examples:
// removNb(26) should return [(15, 21), (21, 15)]
// or
// removNb(26) should return { {15, 21}, {21, 15} }
// or
// removeNb(26) should return [[15, 21], [21, 15]]
// or
// removNb(26) should return [ {15, 21}, {21, 15} ]
// or
// removNb(26) should return "15 21, 21 15"
// or
//
// in C:
// removNb(26) should return  {{15, 21}{21, 15}} tested by way of strings.
// Function removNb should return a pointer to an allocated array of Pair pointers, each one also allocated.
// Note
// See examples of returns for each language in "RUN SAMPLE TESTS"

// assertion failed: `(left == right)`
// left: `[(559756, 893250), (893250, 559756), (550320, 908566), (908566, 550320)]`,
// right: `[(550320, 908566), (559756, 893250), (893250, 559756), (908566, 550320)]`

fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let m = m as i64;
    let mut res = vec![];
    let sum_from_1 = (m + 1) * m / 2;
    let root = (sum_from_1 as f64).sqrt().floor() as i64;
    for i in (1..root).rev() {
        let sum = sum_from_1 - 2 * i;
        let o = sum / i;
        if o > m {
            break;
        }
        if i * o == (sum_from_1 - i - o) {
            res.push((i as i32, o as i32));
            res.push((o as i32, i as i32));
        }
    }
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

fn main() {
    println!("{:?}", remove_nb(26));
    println!("{:?}", remove_nb(1000003));
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
fn basics_remove_nb() {

    testing(26, vec![(15, 21), (21, 15)]);
    testing(100, vec![]);
    testing(101, vec![(55, 91), (91, 55)]);
    testing(102, vec![(70, 73), (73, 70)]);

}

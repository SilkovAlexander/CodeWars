// Create a function that takes a positive integer and returns the next bigger number that can be formed by rearranging its digits. For example:
//
// 12 ==> 21
// 513 ==> 531
// 2017 ==> 2071
// nextBigger(num: 12)   // returns 21
// nextBigger(num: 513)  // returns 531
// nextBigger(num: 2017) // returns 2071
// If the digits can't be rearranged to form a bigger number, return -1 (or nil in Swift):
//
// 9 ==> -1
// 111 ==> -1
// 531 ==> -1
// nextBigger(num: 9)   // returns nil
// nextBigger(num: 111) // returns nil
// nextBigger(num: 531) // returns nil

// digits should be sorted to get sorted output
fn get_permutations(digits: Vec<i64>) -> Vec<Vec<i64>> {
    // let mut digits = digits;
    // digits.sort();
    if digits.len() <= 1 {
        return vec![digits];
    }
    let mut desc_dig = digits.clone();
    desc_dig.sort_by(|a, c| c.cmp(a));
    if desc_dig == digits {
        return vec![digits];
    }
    let mut res = vec![];
    for i in 0..digits.len() {
        if digits[0] > digits[i] {
            continue;
        }
        let mut trimmed = digits.clone();
        trimmed.remove(i);
        let tmp = get_permutations(trimmed);
        for mut v in tmp {
            v.insert(0, digits[i]);
            res.push(v.clone());
        }
    }
    // res.dedup();
    res
}

fn vec_to_num(digits: Vec<i64>) -> i64 {
    i64::from_str_radix(&digits.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(""), 10).unwrap()
}

fn next_bigger_number(n: i64) -> i64 {
    // eprintln!("{}", n);
    let digits = n.to_string()
        .chars()
        .into_iter()
        .map(|c| i64::from_str_radix(&c.to_string(), 10).unwrap())
        .collect::<Vec<i64>>();
    // 12345
    //    54
    // не надо склеивать а толька искать больший суффикс

    for i in 2..=digits.len() {
        let (high, low) = digits.split_at(digits.len() - i);
        let mut low = low.to_vec();
        let init = low.clone();
        low.sort(); // ??? get rid of?
        let permutations = get_permutations(low.clone());
        let index = permutations.binary_search(&init).unwrap();
        let (_, permutations) = permutations.split_at(index+1);
        let permutations = permutations.to_vec();
        println!("{:?} {:?}", init, permutations);
        for mut p in permutations {
            let mut tmp = high.clone().to_vec();
            tmp.append(&mut p);
            let num = vec_to_num(tmp);
            if num > n {
                return num;
            }
        }
    }
    return -1;
}

fn main() {
    // let mut low = vec![8, 9, 0];
    // let (a, b) = low.split_at(1);
    // let mut low = a.to_vec();
    // let mut b = b.to_vec();
    // b.sort();
    // low.append(&mut b);
    // println!("{:?}", low);
    // let per = get_permutations(vec![1, 2, 3, 4, 4]);
    // println!("{} {:?}", per.len(), per);
    // let per = get_permutations(vec![3, 1, 2, 4, 4]);
    // println!("{} {:?}", per.len(), per);
    // let per = get_permutations(vec![1, 2, 3, 4, 5]);
    // println!("{} {:?}", per.len(), per);
    // println!("{:?}", next_bigger_number(1234567890));
    // println!("{:?}", next_bigger_number(988877776665544321));
    // println!("{:?}", next_bigger_number(7665544321));
    println!("{:?}", next_bigger_number(12345));
    // for i in 1..1000000 {
    //     let _ = next_bigger_number(i);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(21, next_bigger_number(12));
        assert_eq!(531, next_bigger_number(513));
        assert_eq!(2071, next_bigger_number(2017));
        assert_eq!(441, next_bigger_number(414));
        assert_eq!(414, next_bigger_number(144));
        assert_eq!(1234567908, next_bigger_number(1234567890));
    }
}

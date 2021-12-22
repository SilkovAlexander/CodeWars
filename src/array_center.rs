// Given an array of ints, return the index such that the sum of the elements to the right of that
// index equals the sum of the elements to the left of that index. If there is no such index, return -1.
// If there is more than one such index, return the left-most index.
//
// For example:
//
// --For Haskell
// peak [1,12,3,3,6,3,1] == Just 2
// peak [10,20,30,40]  == Nothing
// The special case of an array of zeros (for instance [0,0,0,0]) will not be tested.
//
// More examples in the test cases.
//
// Good luck!
//
// Please also try Simple time difference

fn peak(arr: &[u32]) -> Option<usize> {
    let mut sum: u32 = arr.iter().sum();

    // println!("{}", sum);
    for i in 2..arr.len() {
        let s = sum - arr[i];
        if s % 2 == 1 {
            continue
        }
        // println!("{}", s);
        let ss = arr[0..i].iter().sum::<u32>();
        // println!("{}", ss);
        if ss == s >> 1 {
            return Some(i)
        }
    }
    None
}

fn main() {
    println!("{:?}", peak(&vec![1,2,3,5,3,2,1]));
}

#[cfg(test)]
mod tests {
    use super::peak;

    #[test]
    fn basic_tests() {
        assert_eq!(peak(&vec![1,2,3,5,3,2,1]), Some(3));
        assert_eq!(peak(&vec![1,12,3,3,6,3,1]), Some(2));
        assert_eq!(peak(&vec![10,20,30,40]), None);
    }
}

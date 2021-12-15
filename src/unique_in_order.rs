// Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements with the same value next to each other and preserving the original order of elements.
//
// For example:
//
// uniqueInOrder("AAAABBBCCDAABBB") == {'A', 'B', 'C', 'D', 'A', 'B'}
// uniqueInOrder("ABBCcAD")         == {'A', 'B', 'C', 'c', 'A', 'D'}
// uniqueInOrder([1,2,2,3,3])       == {1,2,3}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
    where
        T: std::iter::IntoIterator,
        T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut res: Vec<T::Item> = vec![];
    let mut el = sequence.into_iter();
    loop {
        match el.next() {
            Some(cur) => {
                if !res.is_empty() && res[res.len()-1] == cur {
                    continue;
                }
                res.push(cur);
            }
            _ => break
        }

    }
    res
}

fn main() {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
        assert_eq!(unique_in_order("ABBCcAD".chars()), vec!['A', 'B', 'C', 'c', 'A', 'D']);
        assert_eq!(unique_in_order([1,2,2,3,3]), vec![1,2,3]);
    }
}
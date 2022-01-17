// A format for expressing an ordered list of integers is to use a comma separated list of either
//
// individual integers
// or a range of integers denoted by the starting integer separated from the end integer in the range by a dash, '-'. The range includes all integers in the interval including both endpoints. It is not considered a range unless it spans at least 3 numbers. For example "12,13,15-17"
// Complete the solution so that it takes a list of integers in increasing order and returns a correctly formatted string in the range format.
//
// Example:
//
// solution([-6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20]);
// // returns "-6,-3-1,3-5,7-11,14,15,17-20"
// Courtesy of rosettacode.org


fn main() {
    println!("{:?}", solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));
}

mod solution {

    pub fn range_extraction(a: &[i32]) -> String {
        let mut res = "".to_string();
        if a.len() == 0 {
            return res;
        }
        let mut tmp = vec![a[0]];
        let mut append = |tmp: &Vec<i32>| {
            match tmp.len() {
                1 => { res += &format!(",{}", tmp[0]) },
                2 => { res += &format!(",{},{}", tmp[0], tmp[1]) }
                _ => { res += &format!(",{}-{}", tmp[0], tmp.last().unwrap()) }
            };
        };
        for el in 1..a.len() {
            if tmp.len() == 0 || a[el] == tmp.last().unwrap() + 1 {
                tmp.push(a[el]);
            } else {
                append(&tmp);
                tmp.clear();
                tmp.push(a[el]);
            }
        }
        append(&tmp);
        res.trim_start_matches(|c| c == ',').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));
        assert_eq!("-3--1,2,10,15,16,18-20", solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
    }
}

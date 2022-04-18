// Buddy pairs
// You know what divisors of a number are. The divisors of a positive integer n are said to be proper when you consider only the divisors other than n itself. In the following description, divisors will mean proper divisors. For example for 100 they are 1, 2, 4, 5, 10, 20, 25, and 50.
//
// Let s(n) be the sum of these proper divisors of n. Call buddy two positive integers such that the sum of the proper divisors of each number is one more than the other number:
//
// (n, m) are a pair of buddy if s(m) = n + 1 and s(n) = m + 1
//
// For example 48 & 75 is such a pair:
//
// Divisors of 48 are: 1, 2, 3, 4, 6, 8, 12, 16, 24 --> sum: 76 = 75 + 1
// Divisors of 75 are: 1, 3, 5, 15, 25 --> sum: 49 = 48 + 1
// Task
// Given two positive integers start and limit, the function buddy(start, limit) should return the first pair (n m) of buddy pairs such that n (positive integer) is between start (inclusive) and limit (inclusive); m can be greater than limit and has to be greater than n
//
// If there is no buddy pair satisfying the conditions, then return "Nothing" or (for Go lang) nil or (for Dart) null; (for Lua, Pascal, Perl, D) [-1, -1].


fn main() {
    println!("HEllo");
    let tmp =  find_all_divisors(48);
    println!("{:?}", tmp);
    println!("{:?}", tmp.iter().sum::<i64>());
    println!("{:?}", find_all_divisors(75));

    println!("{:?}", buddy(10, 50));
    println!("{:?}", buddy(1081180, 1103735));
}


fn find_all_divisors(n: i64) -> Vec<i64> {
    if n == 1 {
        return vec![1];
    }

    let mut result = vec![];
    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            if i * i == n {
                result.push(i);
            } else {
                result.push(i);
                result.push(n / i);
            }
        }
    }
    result
}

fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
    for i in start..=limit {
        let buddy: i64 = find_all_divisors(i).iter().sum();
        let buddy_div: i64 = find_all_divisors(buddy).iter().sum();
        if buddy_div == i {
            return Some((i, buddy));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(start: i64, limit: i64, exp: Option<(i64, i64)>) -> () {
        println!("start:{}", start);
        println!("limit:{}", limit);
        let ans = buddy(start, limit);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10, 50,  Some((48, 75)));
        dotest(1081180, 1103735, Some((1081184, 1331967)));

    }
}

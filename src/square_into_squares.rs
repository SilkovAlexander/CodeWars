// My little sister came back home from school with the following task: given a squared sheet of paper she has to cut it in pieces which, when assembled, give squares the sides of which form an increasing sequence of numbers. At the beginning it was lot of fun but little by little we were tired of seeing the pile of torn paper. So we decided to write a program that could help us and protects trees.
//
// Task
// Given a positive integral number n, return a strictly increasing sequence (list/array/string depending on the language) of numbers, so that the sum of the squares is equal to n².
//
// If there are multiple solutions (and there will be), return as far as possible the result with the largest possible values:
//
// Examples
// decompose(11) must return [1,2,4,10]. Note that there are actually two ways to decompose 11², 11² = 121 = 1 + 4 + 16 + 100 = 1² + 2² + 4² + 10² but don't return [2,6,9], since 9 is smaller than 10.
//
// For decompose(50) don't return [1, 1, 4, 9, 49] but [1, 3, 5, 8, 49] since [1, 1, 4, 9, 49] doesn't form a strictly increasing sequence.
//
// Note
// Neither [n] nor [1,1,1,…,1] are valid solutions. If no valid solution exists, return nil, null, Nothing, None (depending on the language) or "[]" (C) ,{} (C++), [] (Swift, Go).
//
// The function "decompose" will take a positive integer n and return the decomposition of N = n² as:
//
// [x1 ... xk] or
// "x1 ... xk" or
// Just [x1 ... xk] or
// Some [x1 ... xk] or
// {x1 ... xk} or
// "[x1,x2, ... ,xk]"
// depending on the language (see "Sample tests")
//
// Note for Bash
// decompose 50 returns "1,3,5,8,49"
// decompose 4  returns "Nothing"
// Hint
// Very often xk will be n-1.

fn _decompose(n: i128, full_square: bool) -> Option<Vec<i64>> {
    if n == 0 {
        return  Some(vec![])
    }

    let root = match full_square {
        true => {
            (n as f64).sqrt().floor() as i64
        },
        false => {
            ((n - 1) as f64).sqrt().floor() as i64
        }
    };
    let rroot = (root as f64).sqrt().floor() as i64;
    for i in (rroot..root+1).rev() {
        match _decompose(n - (i as i128 * i as i128) as i128, true) {
            Some(res) => {
                if !res.is_empty() && res[res.len() - 1] >= i {
                    return None;
                }
                let mut res = res;
                res.insert(0, i);
                res.sort();
                return Some(res);
            },
            None => {}
        }
    }

    None
}

fn decompose(n: i64) -> Option<Vec<i64>> {
    let n2: i128 = n as i128 * n as i128;
    _decompose(n2, false)
}


fn main() {
    println!("{:?}", decompose(44));
    // println!("{:?}", decompose(5));
}


fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
    assert_eq!(decompose(n), exp)
}

#[test]
fn tests_decompose() {

    testing(50, Some(vec![1,3,5,8,49]));
    testing(44, Some(vec![2,3,5,7,43]));
    testing(625, Some(vec![2,5,8,34,624]));
    testing(5, Some(vec![3,4]));

}
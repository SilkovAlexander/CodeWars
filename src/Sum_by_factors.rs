// Given an array of positive or negative integers
//
// I= [i1,..,in]
//
// you have to produce a sorted array P of the form
//
// [ [p, sum of all ij of I for which p is a prime factor (p positive) of ij] ...]
//
// P will be sorted by increasing order of the prime numbers. The final result has to be given as a
// string in Java, C#, C, C++ and as an array of arrays in other languages.
//
// Example:
// I = [12, 15] # result = [(2, 12), (3, 27), (5, 15)]
// [2, 3, 5] is the list of all prime factors of the elements of I, hence the result.
//
// Notes:
//
// It can happen that a sum is 0 if some numbers are negative!
// Example: I = [15, 30, -45] 5 divides 15, 30 and (-45) so 5 appears in the result, the sum of the
// numbers for which 5 is a factor is 0 so we have [5, 0] in the result amongst others.
//
// In Fortran - as in any other language - the returned string is not permitted to contain any
// redundant trailing whitespace: you can use dynamically allocated character strings.


fn generate_primes(limit: i64) -> Vec<i64> {
    match limit {
        0 | 1 => { return vec![]; },
        2 => { return vec![2]; },
        _ => {}
    };
    let mut res = vec![2];
    let mut i = 1;
    loop {
        i += 2;
        if i > limit {
            break;
        }
        let mut flag = true;
        for prime in res.clone().into_iter() {
            if i % prime == 0 {
                flag = false;
                break;
            }
        }
        if flag {
            res.push(i);
        }
    }
    res
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let primes = generate_primes(l.iter().map(|e| e.abs()).max().unwrap_or(0).to_owned());
    let mut res = vec![];
    for p in primes.iter() {
        let mut flag = false;
        let s = l.iter().map(|num| match num % p {
            0 => {flag = true; num},
            _ => &0
        }).sum::<i64>();
        if s != 0 || flag {
            res.push((p.to_owned(), s));
        }
    }
    res
}

fn main() {
    println!("{:?}", sum_of_divided(vec![-12, -15]));
    // println!("{:?}", generate_primes(83));
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

#[test]
fn basics_sum_of_divided() {

    testing(vec![12, 15], vec![ (2, 12), (3, 27), (5, 15) ]);
    testing(vec![15,21,24,30,45], vec![ (2, 54), (3, 135), (5, 90), (7, 21) ]);

}
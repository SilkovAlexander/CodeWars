// 1, 246, 2, 123, 3, 82, 6, 41 are the divisors of number 246. Squaring these divisors we get: 1, 60516, 4, 15129, 9, 6724, 36, 1681. The sum of these squares is 84100 which is 290 * 290.
//
// Task
// Find all integers between m and n (m and n integers with 1 <= m <= n) such that the sum of their squared divisors is itself a square.
//
// We will return an array of subarrays or of tuples (in C an array of Pair) or a string. The subarrays (or tuples or Pairs) will have two elements: first the number the squared divisors of which is a square and then the sum of the squared divisors.
//
// Example:
// list_squared(1, 250) --> [[1, 1], [42, 2500], [246, 84100]]
// list_squared(42, 250) --> [[42, 2500], [246, 84100]]
// The form of the examples may change according to the language, see "Sample Tests".
//
// Note
// In Fortran - as in any other language - the returned string is not permitted to contain any redundant trailing whitespace: you can use dynamically allocated character strings.

fn generate_primes(limit: u64) -> Vec<u64> {
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

fn decompose(num: u64, primes: Option<Vec<u64>>) -> Vec<u64> {
    let mut m_num = num.clone();
    let primes = match primes {
        Some(primes) => primes,
        None => {
            let root = (num as f64).sqrt().floor() as u64;
            generate_primes(root)
        }
    };
    let mut res = vec![];
    for prime in primes.into_iter() {
        while m_num % prime == 0 {
            m_num = m_num / prime;
            res.push(prime);
        }
        if m_num == 1 {
            break;
        }
    }
    if m_num != 1 {
        res.push(m_num)
    }
    res
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let primes = generate_primes((n as f64).sqrt() as u64);
    let mut result = vec![];
    for i in m..n {
        let divisors = decompose(i, Some(primes.clone()));
        let mut res = vec![];
        for mask in 0 as u64..(1 << divisors.len() as u64) {
            let mut tmp = 1;
            for j in 0..divisors.len() {
                if ((1 << j) & mask) != 0 {
                    tmp *= divisors[j];
                }
            }
            res.push(tmp);
        }
        res.sort();
        res.dedup();
        // println!("{:?}", res);
        let mut sqr = 0;
        for r in res.clone() {
            sqr += r * r;
        }
        if (sqr as f64).sqrt().fract() == 0.0 {
            // println!("{:?}", res);
            result.push((i, sqr));
        }
    }
    result
}

// fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
//     (m..=n).map(|i| {
//         (i,
//          (1..=(i as f32).sqrt() as u64).filter_map(|d| {
//              let q = i / d;
//              if q*d == i { Some(d*d + if q == d {0} else {q*q}) } else { None }
//          }).sum())
//     }).filter(|t| {
//         (t.1 as f64).sqrt().fract() == 0.0
//     }).collect::<Vec<_>>()
// }

fn main() {
    println!("{:?}", list_squared(300, 600));
}


fn testing(m: u64, n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(list_squared(m, n), exp)
}

#[test]
fn basics_list_squared() {

    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(1, 250, vec![(1, 1), (42, 2500), (246, 84100)]);
    testing(42, 250, vec![(42, 2500), (246, 84100)]);
    testing(300, 600, vec![]);

}
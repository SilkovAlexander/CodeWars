// ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.
//
// Create a function that takes a string and returns the string ciphered with Rot13. If there are numbers or special characters included in the string, they should be returned as they are. Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".
//

fn generate_primes(limit: i32) -> Vec<i32> {
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


fn prime_decomposition(num: i32, primes: Option<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut m_num = num.clone();
    let primes = match primes {
        Some(primes) => primes,
        None => {
            let root = (num as f64).sqrt().floor() as i32;
            generate_primes(root)
        }
    };
    let mut res = vec![];
    for prime in primes.into_iter() {
        let mut cnt = 0;
        while m_num % prime == 0 {
            m_num = m_num / prime;
            cnt += 1;
        }
        if cnt != 0 {
            res.push((prime as i32, cnt));
        }
        if m_num == 1 {
            break;
        }
    }
    if m_num != 1 {
        res.push((m_num, 1))
    }
    res
}


fn count_kprimes(k: i32, start: i32, nd: i32) -> Vec<i32> {
    let primes = Some(generate_primes(nd));
    _count_kprimes(k, start, nd, primes)
}

fn _count_kprimes(k: i32, start: i32, nd: i32, primes: Option<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    for i in start..nd+1 {
        let tmp = prime_decomposition(i, primes.clone());
        let mut res = 0;
        for (_, cnt) in tmp.into_iter() {
            res += cnt;
        }
        if res == k {
            result.push(i);
        }
    }
    result
}

fn puzzle(s: i32) -> i32 {
    // a + b + c = s, where a is 1-prime, b is 3-prime, and c is 7-prime.
    let primes = Some(generate_primes(s));
    let mut k3 = vec![];
    let mut k7 = vec![];
    for i in 8..s-2 {
        let tmp = prime_decomposition(i, primes.clone());
        let mut res = 0;
        for (_, cnt) in tmp.into_iter() {
            res += cnt;
        }
        if res == 3 {
            k3.push(i);
        }
        if res == 7 {
            k7.push(i);
        }
    }
    if k3.is_empty() || k7.is_empty() {
        return 0;
    }
    let primes = primes.unwrap();
    let mut res = 0;
    for p7 in k7.into_iter() {
        for p3 in k3.clone().into_iter() {
            if primes.contains(&(s - p3 - p7)) {
                res += 1;
            }
        }
    }
    res
}

fn main() {
    println!("{:?}", prime_decomposition(257, None));
    println!("{:?}", prime_decomposition(1040, None));
    println!("{:?}", count_kprimes(5, 1000, 1100));
    println!("{:?}", puzzle(100));
    println!("{:?}", puzzle(144));
    println!("{:?}", puzzle(143));
}

fn testing_count_kprimes(k: i32, start: i32, nd: i32, exp: Vec<i32>) -> () {
    assert_eq!(count_kprimes(k, start, nd), exp)
}
#[test]
fn basics_count_kprimes() {
    testing_count_kprimes(5, 1000, 1100, vec![1020, 1026, 1032, 1044, 1050, 1053, 1064, 1072, 1092, 1100]);
    testing_count_kprimes(12, 100000, 100100, vec![]);
}

fn testing(n: i32, exp: i32) -> () {
    assert_eq!(puzzle(n), exp)
}
#[test]
fn basics_puzzle() {
    testing(100, 0);
    testing(144, 0);
    testing(143, 2);
}

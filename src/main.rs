// Common denominators
//
// You will have a list of rationals in the form
//
// { {numer_1, denom_1} , ... {numer_n, denom_n} }
// or
// [ [numer_1, denom_1] , ... [numer_n, denom_n] ]
// or
// [ (numer_1, denom_1) , ... (numer_n, denom_n) ]
// where all numbers are positive ints. You have to produce a result in the form:
//
// (N_1, D) ... (N_n, D)
// or
// [ [N_1, D] ... [N_n, D] ]
// or
// [ (N_1', D) , ... (N_n, D) ]
// or
// {{N_1, D} ... {N_n, D}}
// or
// "(N_1, D) ... (N_n, D)"
// depending on the language (See Example tests) in which D is as small as possible and
//
// N_1/D == numer_1/denom_1 ... N_n/D == numer_n,/denom_n.
// Example:
// convertFracs [(1, 2), (1, 3), (1, 4)] `shouldBe` [(6, 12), (4, 12), (3, 12)]
// Note:
// Due to the fact that the first translations were written long ago - more than 6 years - these first translations have only irreducible fractions.
//
// Newer translations have some reducible fractions. To be on the safe side it is better to do a bit more work by simplifying fractions even if they don't have to be.
//
// Note for Bash:
// input is a string, e.g "2,4,2,6,2,8" output is then "6 12 4 12 3 12"


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


fn prime_decomposition(num: i64, primes: &Vec<i64>) -> Vec<(i64, i64)> {
    let mut m_num = num.clone();
    let mut res = vec![];
    for &prime in primes.into_iter() {
        let mut cnt = 0;
        while m_num % prime == 0 {
            m_num = m_num / prime;
            cnt += 1;
        }
        if cnt != 0 {
            res.push((prime as i64, cnt));
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


fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let denoms: Vec<i64> = l.iter().map(|pair| pair.1).collect();
    let primes = generate_primes(denoms.iter().max().unwrap().to_owned());
    let denoms = denoms
        .iter()
        .map(|&d| prime_decomposition(d, &primes))
        .collect::<Vec<Vec<(i64, i64)>>>();
    let mut common: Vec<(i64, i64)> = vec![];
    for delimiters in denoms {
        for d in delimiters {
            let index = common
                .iter()
                .position(|&p| p.0 == d.0);
            match index {
                Some(index) => { common[index].1 = i64::max(common[index].1, d.1); }
                None => { common.push(d); }
            };
        }
    };
    let mut common_delimiter = 1;
    for c in common {
        common_delimiter *= c.0.pow(c.1 as u32);
    }
    l.iter().map(|&pair| {
        (common_delimiter * pair.0 / pair.1, common_delimiter)
    }).collect()
}


fn main() {
    println!("{:?}", convert_fracts(vec![(69, 130), (87, 1310), (3, 4)]));
    println!("{:?}", convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)]));
}

fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(convert_fracts(l), exp)
}

#[test]
fn basics_convert_fracts() {

    testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);

}

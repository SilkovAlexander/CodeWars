// The Fibonacci numbers are the numbers in the following integer sequence (Fn):
//
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, ...
//
// such as
//
// F(n) = F(n-1) + F(n-2) with F(0) = 0 and F(1) = 1.
//
// Given a number, say prod (for product), we search two Fibonacci numbers F(n) and F(n+1) verifying
//
// F(n) * F(n+1) = prod.
//
// Your function productFib takes an integer (prod) and returns an array:
//
// [F(n), F(n+1), true] or {F(n), F(n+1), 1} or (F(n), F(n+1), True)
// depending on the language if F(n) * F(n+1) = prod.
//
// If you don't find two consecutive F(n) verifying F(n) * F(n+1) = prodyou will return
//
// [F(n), F(n+1), false] or {F(n), F(n+1), 0} or (F(n), F(n+1), False)
// F(n) being the smallest one such as F(n) * F(n+1) > prod.
//
// Some Examples of Return:
// (depend on the language)
//
// productFib(714) # should return (21, 34, true),
// # since F(8) = 21, F(9) = 34 and 714 = 21 * 34
//
// productFib(800) # should return (34, 55, false),
// # since F(8) = 21, F(9) = 34, F(10) = 55 and 21 * 34 < 800 < 34 * 55
// -----
// productFib(714) # should return [21, 34, true],
// productFib(800) # should return [34, 55, false],
// -----
// productFib(714) # should return {21, 34, 1},
// productFib(800) # should return {34, 55, 0},
// -----
// productFib(714) # should return {21, 34, true},
// productFib(800) # should return {34, 55, false},
// Note:
// You can see examples for your language in "Sample Tests".


fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

struct Fib {
    vals: Vec<u64>,
}

impl Fib {
    fn get(&mut self, n: u64) -> u64 {
        let n = n as usize;
        let l = self.vals.len();
        if n >= l {
            for i in l..=n {
                self.vals.push(fib(i as u64))
            }
        }
        self.vals[n]
    }
}

fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut fibbonizer = Fib { vals: vec![] };
    let root = (prod as f64).sqrt() as u64;
    let mut it = 0;
    loop {
        if fibbonizer.get(it) > root {
            break;
        }
        it += 1;
    }
    let cur_f = fibbonizer.get(it);
    let prev_f = fibbonizer.get(it - 1);
    if cur_f * prev_f == prod {
        (prev_f, cur_f, true)
    } else if cur_f * prev_f < prod {
        (cur_f, fibbonizer.get(it + 1), false)
    } else {
        (prev_f, cur_f, false)
    }
}

fn main() {
    // let mut prev = 1;
    // for i in 0..30 {
    //     let f = fib(i);
    //     println!("{} {}", f, if prev != 0 {f as f64 / prev as f64} else {0.0} );
    //     prev = f;
    // }
    // println!();
    // println!("{:?}", product_fib(4895));
    // println!("{:?}", product_fib(5895));
    for i in 1..100 {
        product_fib(100000000000 * i);
    }
}

fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
    assert_eq!(product_fib(prod), exp)
}

#[test]
fn basics_product_fib() {
    dotest(4895, (55, 89, true));
    dotest(5895, (89, 144, false));
}
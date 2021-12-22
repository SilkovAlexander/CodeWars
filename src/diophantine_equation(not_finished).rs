// In mathematics, a Diophantine equation is a polynomial equation, usually with two or more unknowns, such that only the integer solutions are sought or studied.
//
// In this kata we want to find all integers x, y (x >= 0, y >= 0) solutions of a diophantine equation of the form:
//
// x2 - 4 * y2 = n
// (where the unknowns are x and y, and n is a given positive number) in decreasing order of the positive xi.
//
// If there is no solution return [] or "[]" or "". (See "RUN SAMPLE TESTS" for examples of returns).
//
// Examples:
// solEquaStr(90005) --> "[[45003, 22501], [9003, 4499], [981, 467], [309, 37]]"
// solEquaStr(90002) --> "[]"
// Hint:
// x2 - 4 * y2 = (x - 2*y) * (x + 2*y)

fn solequa(n: u64) -> Vec<(u64, u64)> {
    // y < n/2
    // x < n, x > 2 * y
    // let root = (n as f64).sqrt().floor() as u64;
    let mut res: Vec<(u64, u64)> = [].to_vec();

    for y in 0..(n / 2 as u64) {
        let approximate = (n * n - 4 * y * y) as f64.sqrt().floor() as u64;
        for x in approximate..n {
            let diff = x * x - 4 * y * y;
            if diff == n {
                res.insert(0, (x, y));
            } else if diff > n {
                break;
            }
        }
    }
    res
}


fn main() {
    println!("{:?}", solequa(20));
    println!("{:?}", solequa(9004));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(solequa(n), exp)
    }

    #[test]
    fn basics_solequa() {
        testing(5, vec![(3, 1)]);
        testing(20, vec![(6, 2)]);
        testing(9001, vec![(4501, 2250)]);
        testing(9004, vec![(2252, 1125)]);
        testing(9005, vec![(4503, 2251), (903, 449)]);
    }
}
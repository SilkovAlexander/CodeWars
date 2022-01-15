
// The drawing shows 6 squares the sides of which have a length of 1, 1, 2, 3, 5, 8. It's easy to see that the sum of the perimeters of these squares is : 4 * (1 + 1 + 2 + 3 + 5 + 8) = 4 * 20 = 80
//
// Could you give the sum of the perimeters of all the squares in a rectangle when there are n + 1 squares disposed in the same manner as in the drawing:
//
// alternative text
//
// Hint:
// See Fibonacci sequence
//
// Ref:
// http://oeis.org/A000045
//
// The function perimeter has for parameter n where n + 1 is the number of squares (they are numbered from 0 to n) and returns the total perimeter of all the squares.
//
// perimeter(5)  should return 80
// perimeter(7)  should return 216


fn main() {
    println!("{:?}", perimeter(5));
}

/*
Fn = Fn-1 + Fn-2
Sum(Fn)= Fn-1 + Fn-2 + Fn-2 + Fn-3 
 */

fn perimeter(n: u64) -> u64 {
    let mut fib = vec![1, 1];
    for i in 2..=n as usize {
        fib.push(fib[i-2] + fib[i-1]);
    }
    fib.iter().sum::<u64>() * 4
}

fn dotest(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
    dotest(5, 80);
    dotest(7, 216);
    dotest(20, 114624);
    dotest(30, 14098308);
}
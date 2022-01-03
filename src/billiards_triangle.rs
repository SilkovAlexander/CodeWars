// Remember the triangle of balls in billiards? To build a classic triangle (5 levels) you need 15 balls. With 3 balls you can build a 2-level triangle, etc.
//
// For more examples,
//
// pyramid(1) == 1
//
// pyramid(3) == 2
//
// pyramid(6) == 3
//
// pyramid(10) == 4
//
// pyramid(15) == 5
// Write a function that takes number of balls (≥ 1) and calculates how many levels you can build a triangle.


fn pyramid(balls: u16) -> u16 {
    ((((balls as f64) * 8.0 + 1.0).sqrt() - 1.0) / 2.0).floor() as u16
}


fn main() {
    println!("{:?}", pyramid(1));
    println!("{:?}", pyramid(100));
    println!("{:?}", pyramid(9999));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pyramid(1), 1);
        assert_eq!(pyramid(4), 2);
        assert_eq!(pyramid(20), 5);
        assert_eq!(pyramid(100), 13);
        assert_eq!(pyramid(2211), 66);
        assert_eq!(pyramid(9999), 140);
    }
}
// The rgb function is incomplete. Complete it so that passing in RGB decimal values will result in a hexadecimal representation being returned. Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.
//
// Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.
//
// The following are examples of expected output values:
//
// kata.rgb(255, 255, 255) -- returns FFFFFF
// kata.rgb(255, 255, 300) -- returns FFFFFF
// kata.rgb(0, 0, 0) -- returns 000000
// kata.rgb(148, 0, 211) -- returns 9400D3


fn rgb(r: i32, g: i32, b: i32) -> String {
    fn convert(i: i32) -> i32 {
        if i < 0 { return 0; }
        if i > 255 { return 255; }
        i
    }
    format!("{:02X}{:02X}{:02X}", convert(r), convert(g), convert(b))
}


fn main() {
}

macro_rules! compare {
  ( $got : expr, $expected : expr ) => {
    if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
  };
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}

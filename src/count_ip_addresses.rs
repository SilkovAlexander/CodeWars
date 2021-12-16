// Implement a function that receives two IPv4 addresses, and returns the number of addresses between them (including the first one, excluding the last one).
//
// All inputs will be valid IPv4 addresses in the form of strings. The last address will always be greater than the first one.
//
// Examples
// ips_between("10.0.0.0", "10.0.0.50") ==  50
// ips_between("10.0.0.0", "10.0.1.0")  == 256
// ips_between("20.0.0.10", "20.0.1.0") == 246

fn convert_ip(ip: &str) -> u32 {
    let quarters: Vec<&str> = ip.split('.').collect();
    let mut res = 0;
    for val in quarters {
        res = (res << 8) + u32::from_str_radix(val, 10).unwrap_or(0);
    }
    res
}

fn ips_between(start: &str, end: &str) -> u32 {
    convert_ip(end) - convert_ip(start)
}

fn main() {
    println!("{}", ips_between("10.0.0.0", "10.0.0.50"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
        assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
    }
}



fn digitize(n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }
    let mut res: Vec<u8> = Vec::new();
    let mut n = n;
    while n != 0 {
        res.push((n % 10) as u8);
        n /= 10;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
        assert_eq!(digitize(0), vec![0]);
        assert_eq!(digitize(1111), vec![1,1,1,1]);
    }
}
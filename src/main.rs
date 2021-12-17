// John and his wife Ann have decided to go to Codewars.
//
// On first day Ann will do one kata and John - he wants to know how it is working - 0 kata.
//
// Let us call a(n) - and j(n) - the number of katas done by Ann - and John - at day n. We have a(0) = 1 and in the same manner j(0) = 0.
//
// They have chosen the following rules:
//
// On day n the number of katas done by Ann should be n minus the number of katas done by John at day t, t being equal to the number of katas done by Ann herself at day n - 1.
//
// On day n the number of katas done by John should be n minus the number of katas done by Ann at day t, t being equal to the number of katas done by John himself at day n - 1.
//
// Whoops! I think they need to lay out a little clearer exactly what there're getting themselves into!
//
// Could you write:
// two functions ann and john (parameter n) giving the list of the numbers of katas Ann and John should take on the first n days (see first examples below)?
// The total number of katas taken by ann function sum_ann(n) and john function sum_john(n) - on the first n days?
// The functions in 1) are not tested in Fortran and not tested in Shell.
//
// Examples:
// john(11) -->  [0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]
// ann(6) -->  [1, 1, 2, 2, 3, 3]
//
// sum_john(75) -->  1720
// sum_ann(150) -->  6930
// Shell Note:
// sumJohnAndAnn has two parameters:
//
// first one : n (number of days, $1)
//
// second one : which($2) ->
//
// 1 for getting John's sum
//
// 2 for getting Ann's sum.
//
// See "Sample Tests".
//
// Note:
// Keep an eye on performance.

struct a {
    vals: Vec<i32>
}

impl a {
    fn get(&mut self, n: i32, jj: &mut j) -> i32{
        if n < self.vals.len() as i32 {
            return self.vals[n as usize];
        }
        let val = match n {
            0 => 1,
            n => {
                let ind = self.get(n - 1, jj);
                n - jj.get(ind, self)
            }
        };
        self.vals.push(val);
        val
    }
}

struct j {
    vals: Vec<i32>
}

impl j {
    fn get(&mut self, n: i32, aa: &mut a) -> i32{
        if n < self.vals.len() as i32 {
            return self.vals[n as usize];
        }
        let val = match n {
            0 => 1,
            n => {
                let ind = self.get(n - 1, aa);
                n - aa.get(ind, self)
            }
        };
        self.vals.push(val);
        val
    }
}


fn john(n: i32) -> Vec<i32> {
    let mut aa = a { vals: [1].to_vec() };
    let mut jj = j { vals: [0].to_vec() };
    jj.get(n-1, &mut aa);
    jj.vals

}

fn ann(n: i32) -> Vec<i32> {
    let mut aa = a { vals: [1].to_vec() };
    let mut jj = j { vals: [0].to_vec() };
    aa.get(n-1, &mut jj);
    aa.vals
}

fn sum_john(n: i32) -> i32 {
    let mut res = 0;
    let v = john(n);
    for el in v {
        res += el;
    }
    res
}

fn sum_ann(n: i32) -> i32 {
    let mut res = 0;
    let v = ann(n);
    for el in v {
        res += el;
    }
    res
}


fn main() {
    println!("{:?}", john(11));
    println!("{:?}", ann(6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_john() {
        assert_eq!(john(11), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
        assert_eq!(john(14), vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
    }
    #[test]
    fn test_ann() {
        assert_eq!(ann(6), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(ann(15), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
    }
    #[test]
    fn test_sum_john() {
        assert_eq!(sum_john(75), 1720);
        assert_eq!(sum_john(78), 1861);
    }
    #[test]
    fn test_sum_ann() {
        assert_eq!(sum_ann(115), 4070);
        assert_eq!(sum_ann(150), 6930);
    }
}
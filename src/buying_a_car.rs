// Let us begin with an example:
//
// A man has a rather old car being worth $2000. He saw a secondhand car being worth $8000. He wants to keep his old car until he can buy the secondhand one.
//
// He thinks he can save $1000 each month but the prices of his old car and of the new one decrease of 1.5 percent per month. Furthermore this percent of loss increases of 0.5 percent at the end of every two months. Our man finds it difficult to make all these calculations.
//
// Can you help him?
//
// How many months will it take him to save up enough money to buy the car he wants, and how much money will he have left over?
//
// Parameters and return of function:
//
// parameter (positive int or float, guaranteed) startPriceOld (Old car price)
// parameter (positive int or float, guaranteed) startPriceNew (New car price)
// parameter (positive int or float, guaranteed) savingperMonth
// parameter (positive float or int, guaranteed) percentLossByMonth
//
// nbMonths(2000, 8000, 1000, 1.5) should return [6, 766] or (6, 766)
// Detail of the above example:
// end month 1: percentLoss 1.5 available -4910.0
// end month 2: percentLoss 2.0 available -3791.7999...
// end month 3: percentLoss 2.0 available -2675.964
// end month 4: percentLoss 2.5 available -1534.06489...
// end month 5: percentLoss 2.5 available -395.71327...
// end month 6: percentLoss 3.0 available 766.158120825...
// return [6, 766] or (6, 766)
// where 6 is the number of months at the end of which he can buy the new car and 766 is the nearest integer to 766.158... (rounding 766.158 gives 766).
//
// Note:
//
// Selling, buying and saving are normally done at end of month. Calculations are processed at the end of each considered month but if, by chance from the start, the value of the old car is bigger than the value of the new one or equal there is no saving to be made, no need to wait so he can at the beginning of the month buy the new car:
//
// nbMonths(12000, 8000, 1000, 1.5) should return [0, 4000]
// nbMonths(8000, 8000, 1000, 1.5) should return [0, 0]
// We don't take care of a deposit of savings in a bank:-)


fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    if old > new {
        return (0, old - new);
    }
    let mut req = (new - old) as f64;
    let mut months_cnt = 0;
    let mut perc = perc;
    while req > (months_cnt * saving) as f64 {
        months_cnt += 1;
        if months_cnt % 2 == 0 {
            perc += 0.5;
        }
        let mul_perc = 1.0 - perc / 100.0;
        req = (req * (1.0 - perc / 100.0));
    }
    (months_cnt, ((months_cnt * saving) as f64  - req).round() as i32)
}

fn main() {
    nb_months(7500, 32000, 300, 1.55);
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
}

#[test]
fn basics_nb_months() {
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(12000, 8000, 1000, 1.5 , (0, 4000));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
}

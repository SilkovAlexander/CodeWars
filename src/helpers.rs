


pub fn print_by_lines<T: std::fmt::Debug>(value: &Vec<T>) {
    for line in value {
        println!("{line:?}");
    }
}

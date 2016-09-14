extern crate chrono;
use chrono::*;

fn convert(date_str: &String) -> String {
    let binary  = format!("{:b}", date_str.parse::<i32>().unwrap());
    let rev_str = binary.chars().rev().collect::<String>();
    format!("{}", i32::from_str_radix(rev_str.as_ref(), 2).unwrap())
}

fn main() {
    let start = Local.ymd(1964, 10, 10);
    let end   = Local.ymd(2020,  7, 24);

    let mut now = start.clone();

    while now <= end {
        let formatted = now.format("%Y%m%d").to_string();
        if formatted == convert(&formatted) {
            println!("{:?} -> {:?}", now, formatted);
        }
        now = now + Duration::days(1);
    }
}

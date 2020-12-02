use std::fs::read_to_string;

mod lib;
use lib::*;

fn main() {
    let content = read_to_string("input").unwrap();
    let report: ExpenseReport = content.parse().unwrap();

    println!("Code: {}", report.check());
}

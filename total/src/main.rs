use regex::Regex;
use std::io::{self, prelude::*};

fn main() {
    let digits: Regex = Regex::new(r"\d+").unwrap();
    let (count, sum, min, max) = io::stdin().lock().lines()
        .flat_map( |line|
            digits.find_iter(line.expect("Error reading from stdin").as_str())
                .map( |chars| chars.as_str().parse::<i64>().expect("Number is too large") )
                .collect::<Vec<_>>()    ////////////////  CAN WE AVOID THIS?
        )
        .fold( (0i64, 0i64, i64::MAX, i64::MIN), |(count, sum, min, max), n|
            (count + 1, sum + n, min.min(n), max.max(n))
        );
    println!("count = {}, sum = {}", count, sum);
    if count > 0 {
        println!("min = {}, max = {}, avg = {}", min, max, sum as f64 / count as f64);
    }
}

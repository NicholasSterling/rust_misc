use std::sync::Arc;
use std::thread;

// use std::time;
// use rand::prelude::*;
//thread::sleep(time::Duration::from_millis(random::<u64>() % 300));

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    let joinhandles = (0..8).map( |offset| {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move || {
            let sum: u32 = child_numbers.iter()
                .skip(offset)
                .step_by(5)
                .sum();
            println!("Sum of offset {} is {}", offset, sum);
        } )
    } ).collect::<Vec<_>>();

    /*
    let mut joinhandles = Vec::new();
    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    */

    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
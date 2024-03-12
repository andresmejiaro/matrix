mod complex;
mod errors;
mod matrix;
mod tests;
mod traits;
mod vector;

use std::io;
use tests::*;

fn main() {
    let examples = vec![
        ex00_test, ex01_test, ex02_test, ex03_test, ex04_test, ex05_test,
        ex06_test, ex07_test, ex08_test, ex09_test, ex10_test, ex11_test,
        ex12_test, ex13_test, ex14_test, ex15_test, ex16_test, old_test,
    ];
    loop {
        println!("Enter a number of exercise to run the test for that part of the subject");
        println!("Leave empty to exit!");
        let mut input_string = String::new();
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {}
            Err(_e) => {
                println!("Error reading line...");
                continue;
            }
        }
        let trimmed = input_string.trim();
        if trimmed.is_empty() {
            println!("Bye!");
            break;
        }
        let n = match usize::from_str_radix(trimmed, 10) {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number");
                continue;
            }
        };
        big_line();
        if n >= examples.len() {
            println!("out of bounds try again");
            continue;
        }
        let error = examples[n]();
        match error {
            Ok(_) => (),
            Err(e) => println!("Linear algebra Error: {}", e),
        }
    }
}

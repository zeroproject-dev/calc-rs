mod eval;

use eval::eval;
use std::io::{self, Write};

fn main() {
    println!("Enter a statement:");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        if let Ok(result) = eval(input) {
            println!("{}", result);
        } else {
            println!("Invalid statement");
        }
    }
}

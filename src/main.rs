use expressive_calc;
use std::io::{self, Write};

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input == "exit" {
                    break;
                }
                let result = expressive_calc::evaluate(input);
                match result {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

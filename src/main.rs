use expressive_calc;
use std::io::{self, Write};

fn main() {
    println!("Enter 'exit' to close the calculator.");

    let mut calculator = expressive_calc::Calculator::new();
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
                let result = calculator.evaluate(input);
                match result {
                    Ok((name, value)) => println!("{} = {}", name, value),
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

use std::io::{self, Write};

fn main() {
    loop {
        let mut expression = String::new();
        println!("Enter an expression (or 'exit' to quit):");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read line");

        let expression = expression.trim();
        if expression == "exit" {
            break;
        }

        match calculate(expression) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        };
    }
}

fn calculate(expression: &str) -> Result<f64, &str> {
    let mut current_op: Option<char> = None;
    let mut parts = expression.split_whitespace();
    let mut result = 0.0;

    while let Some(part) = parts.next() {
        if part.chars().all(|c| c.is_digit(10) || c == '.') {
            let number: f64 = part.parse().map_err(|_| "Invalid number")?;
            result = match current_op {
                Some('+') => result + number,
                Some('-') => result - number,
                Some('*') => result * number,
                Some('/') => result / number,
                None => number,
                _ => return Err("Invalid operation"),
            };
        } else if "+-*/".contains(part) && part.len() == 1 {
            current_op = Some(part.chars().next().unwrap());
        } else {
            return Err("Invalid input");
        }
    }
    Ok(result)
}

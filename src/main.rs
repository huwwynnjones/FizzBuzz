use std::io::{self, BufRead};

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    match stdin.lock().read_line(&mut line) {
        Ok(_) => println!("{}", output_string(&line)),
        Err(err) => println!("Unable to read in from STDIN: {}", err),
    };
}

fn output_string(input: &str) -> String {
    let mut output = String::new();
    match validate_input(input) {
        Ok(nmb) => {
            for i in 1..=nmb {
                if i % 5 == 0 && i % 3 == 0 {
                    output.push_str("FizzBuzz\n")
                } else if i % 3 == 0 {
                    output.push_str("Fizz\n")
                } else if i % 5 == 0 {
                    output.push_str("Buzz\n")
                } else {
                    output.push_str(&format!("{}\n", i))
                }
            }
        }
        Err(s) => output.push_str(&s),
    }
    output
}

fn validate_input(input: &str) -> Result<u32, String> {
    match input.trim().parse::<u32>() {
        Ok(nmb) => {
            if (nmb > 0) && (nmb < 107) {
                Ok(nmb)
            } else {
                Err(format!(
                    "The number must be between 0 and 107 exclusive, you gave: {}",
                    nmb
                ))
            }
        }
        Err(_) => Err(format!(
            "The input must be a number between 0 and 107 exclusive, you gave: {}",
            input
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_string() {
        assert_eq!(output_string("3"), "1\n2\nFizz\n");
        assert_eq!(output_string("5"), "1\n2\nFizz\n4\nBuzz\n");
        assert_eq!(
            output_string("15"),
            "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n"
        );
    }

    #[test]
    fn test_validate_input() {
        assert_eq!(validate_input("1"), Ok(1));
        assert_eq!(
            validate_input("0"),
            Err("The number must be between 0 and 107 exclusive, you gave: 0".to_string())
        );
        assert_eq!(
            validate_input("107"),
            Err("The number must be between 0 and 107 exclusive, you gave: 107".to_string())
        );
        assert_eq!(
            validate_input("cat"),
            Err(
                "The input must be a number between 0 and 107 exclusive, you gave: cat".to_string()
            )
        );
    }
}

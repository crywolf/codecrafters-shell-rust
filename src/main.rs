#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        let mut iter = input.split_ascii_whitespace();

        let command = iter.next().unwrap_or_default();
        if command.is_empty() {
            continue;
        }
        let args = iter.collect::<Vec<_>>();

        if command == "exit" {
            std::process::exit(args.first().unwrap_or(&"0").parse::<i32>().unwrap());
        }

        println!("{}: command not found", input);
    }
}

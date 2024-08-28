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

        let command: &str;
        let mut args_str = "";
        let mut args: Option<Vec<&str>> = None;
        if let Some((cmd, args_s)) = input.split_once(' ') {
            command = cmd;
            args_str = args_s;
            args = Some(args_s.split_ascii_whitespace().collect::<Vec<_>>());
        } else {
            command = input;
        }

        if command.is_empty() {
            continue;
        }

        if command == "exit" {
            let code = if let Some(args) = args {
                args.first().unwrap_or(&"0").parse::<i32>().unwrap()
            } else {
                0
            };
            std::process::exit(code);
        }
        if command == "echo" {
            println!("{}", args_str);
            continue;
        }

        println!("{}: command not found", input);
    }
}

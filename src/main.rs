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

        if input.is_empty() {
            continue;
        }

        let cmd = Command::parse(input);

        cmd.invoke();
    }
}

enum Command<'a> {
    Exit(i32),
    Echo(&'a str),
    Unknown(&'a str),
}

impl<'a> Command<'a> {
    pub fn parse(input: &'a str) -> Self {
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

        match command {
            "exit" => {
                let code = args
                    .unwrap_or(vec!["0"])
                    .first()
                    .unwrap()
                    .parse::<i32>()
                    .map_err(|err| {
                        eprintln!("Argument Error: {}", err);
                        err
                    })
                    .unwrap();
                Command::Exit(code)
            }
            "echo" => Command::Echo(args_str),
            _ => Command::Unknown(input),
        }
    }

    pub fn invoke(self) {
        match self {
            Command::Exit(code) => std::process::exit(code),
            Command::Echo(arg) => println!("{}", arg),
            Command::Unknown(cmd) => println!("{}: command not found", cmd),
        }
    }
}

use std::collections::HashSet;
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

#[derive(PartialEq, Eq, Hash, Debug)]
enum Command<'a> {
    Exit(i32),
    Echo(&'a str),
    Type(&'a str),
    Unknown(&'a str),
}

impl<'a> Command<'a> {
    const EXIT: &'static str = "exit";
    const ECHO: &'static str = "echo";
    const TYPE: &'static str = "type";

    const BUILTINS: [&'static str; 3] = [Self::EXIT, Self::ECHO, Self::TYPE];

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
            Self::EXIT => {
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
            Self::ECHO => Command::Echo(args_str),
            Self::TYPE => Command::Type(args_str),
            _ => Command::Unknown(input),
        }
    }

    pub fn invoke(self) {
        match self {
            Command::Exit(code) => std::process::exit(code),
            Command::Echo(arg) => println!("{}", arg),
            Command::Type(arg) => println!("{}{}", arg, Self::cmd_type(arg)),
            Command::Unknown(cmd) => println!("{}: command not found", cmd),
        }
    }

    fn cmd_type(arg: &str) -> String {
        let builtins = HashSet::from(Self::BUILTINS);
        if builtins.contains(arg) {
            return String::from(" is a shell builtin");
        }
        String::from(": not found")
    }
}

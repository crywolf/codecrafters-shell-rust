use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{collections::HashSet, process};

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
    Type(&'a str),
    Pwd(&'a str),
    Cd(&'a str),
    Unknown(&'a str, Option<Vec<&'a str>>),
}

impl<'a> Command<'a> {
    const EXIT: &'static str = "exit";
    const ECHO: &'static str = "echo";
    const TYPE: &'static str = "type";
    const PWD: &'static str = "pwd";
    const CD: &'static str = "cd";

    const BUILTINS: [&'static str; 5] = [Self::EXIT, Self::ECHO, Self::TYPE, Self::PWD, Self::CD];

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
            Self::PWD => Command::Pwd(args_str),
            Self::CD => Command::Cd(args_str),
            _ => Command::Unknown(command, args),
        }
    }

    pub fn invoke(self) {
        match self {
            Command::Exit(code) => std::process::exit(code),
            Command::Echo(arg) => println!("{}", arg),
            Command::Type(arg) => println!("{}{}", arg, Self::cmd_type(arg)),
            Command::Pwd(arg) => {
                if !arg.is_empty() {
                    println!("unknown argument: {}", arg);
                    return;
                }
                let path = Path::new(".");
                println!("{}", path.canonicalize().unwrap().display());
            }
            Command::Cd(args) => {
                let result = if args == "~" {
                    let home =
                        std::env::var("HOME").expect("Could not read HOME environment variable");
                    std::env::set_current_dir(&home)
                } else {
                    std::env::set_current_dir(args)
                };

                match result {
                    Ok(_) => {}
                    Err(err) if err.kind() == io::ErrorKind::NotFound => {
                        eprintln!("{}: {}: No such file or directory", Self::CD, args)
                    }
                    Err(err) => eprintln!("{}: {}: {}", Self::CD, args, err),
                }
            }
            Command::Unknown(cmd, args) => {
                // try to execute it, if it is executable
                if Self::find_in_path(cmd).is_some() {
                    // execute the command
                    let output = process::Command::new(cmd)
                        .args(args.unwrap_or_default())
                        .output()
                        .unwrap_or_else(|_| panic!("{cmd} command failed to start"));
                    io::stdout().write_all(&output.stdout).unwrap();
                } else {
                    println!("{}: command not found", cmd)
                }
            }
        }
    }

    fn cmd_type(arg: &str) -> String {
        // is builtin?
        let builtins = HashSet::from(Self::BUILTINS);
        if builtins.contains(arg) {
            return String::from(" is a shell builtin");
        }
        // is binary (in PATH env var)?
        if let Some(path) = Self::find_in_path(arg) {
            return format!(" is {}", path.display());
        }
        String::from(": not found")
    }

    fn find_in_path(cmd: &str) -> Option<PathBuf> {
        let path_env = std::env::var("PATH").expect("Could not read PATH environment variable");
        let paths: Vec<PathBuf> = path_env.split(":").map(PathBuf::from).collect();

        for mut path in paths {
            path.push(cmd);
            if let Ok(meta) = std::fs::metadata(&path) {
                if meta.is_file() {
                    return Some(path);
                }
            }
        }
        None
    }
}

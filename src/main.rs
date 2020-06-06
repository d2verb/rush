extern crate nix;

use nix::sys::wait::*;
use nix::unistd::*;
use std::env;
use std::ffi::CString;
use std::io::{self, Write};

fn prompt(prefix: &str) -> String {
    print!("{} ", prefix);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");

    line.trim().to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum CommandKind {
    Exit,
    External,
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    kind: CommandKind,
    args: Vec<String>,
}

impl Command {
    fn new(kind: CommandKind, args: Vec<String>) -> Self {
        Command { kind, args }
    }
}

fn get_command(line: String) -> Command {
    let args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    let kind: CommandKind = match args[0].as_str() {
        "exit" => CommandKind::Exit,
        _ => CommandKind::External,
    };
    Command::new(kind, args)
}

fn execve_wrapper(args: Vec<String>) {
    let mut cargs = Vec::<CString>::new();
    for arg in args {
        cargs.push(CString::new(arg).unwrap());
    }

    let envs: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();

    execve(&cargs[0], &cargs[1..], &envs).expect(&format!("failed to execute {:?}", &cargs[0]));
}

fn exec_cmd(cmd: Command) {
    match cmd.kind {
        CommandKind::Exit => {
            std::process::exit(0);
        }
        CommandKind::External => match fork().expect("fork failed") {
            ForkResult::Parent { child } => {
                let _ = waitpid(child, None);
            }
            ForkResult::Child => execve_wrapper(cmd.args),
        },
    }
}

fn main() {
    loop {
        let line = prompt("$");
        let cmd = get_command(line);
        exec_cmd(cmd);
    }
}

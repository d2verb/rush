extern crate nix;

use nix::sys::wait::*;
use nix::unistd::*;
use std::ffi::CString;
use std::io::{self, Write};

fn prompt(prefix: &str) -> std::io::Result<String> {
    print!("{} ", prefix);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    Ok(line.trim().to_string())
}

fn main() {
    loop {
        let line = match prompt("$") {
            Ok(line) => line,
            Err(err) => {
                println!("{}", err);
                break;
            }
        };

        match fork().expect("fork failed") {
            ForkResult::Parent { child } => {
                let _ = waitpid(child, None);
            }
            ForkResult::Child => {
                let mut argv = Vec::<CString>::new();
                for arg in line.split_whitespace() {
                    argv.push(CString::new(arg).unwrap());
                }
                execv(&argv[0], &argv[1..]);
            }
        }
    }
}

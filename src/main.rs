extern crate nix;

use nix::sys::wait::*;
use nix::unistd::*;
use std::env;
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

                let envs: Vec<CString> = env::vars()
                    .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
                    .collect();

                execve(&argv[0], &argv[1..], &envs)
                    .expect(format!("failed to execute command {:?}", argv[0]));
            }
        }
    }
}

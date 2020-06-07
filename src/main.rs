use nix::sys::wait::*;
use nix::unistd::*;
use rush::builtin;
use rush::command::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::ffi::CString;
use std::path::Path;

/// Find real path of given command.
///
/// # Examples
///
/// ```rust
/// let path = find_realpath("sh");
/// assert_eq!(&path, "/bin/sh");
/// ```
fn find_realpath(cmd_name: &str) -> String {
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let cmd_path = Path::new(&path).join(cmd_name);
                if cmd_path.exists() {
                    return cmd_path.to_str().unwrap().to_string();
                }
            }
            cmd_name.to_string()
        }
        None => cmd_name.to_string(),
    }
}

fn execve_wrapper(args: Vec<&str>) {
    let path = CString::new(find_realpath(&args[0])).unwrap();

    let mut cargs = Vec::<CString>::new();
    for arg in args {
        cargs.push(CString::new(arg).unwrap());
    }

    let envs: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();

    execve(&path, &cargs[0..], &envs).expect(&format!("failed to execute {:?}", &cargs[0]));
}

fn execute(cmd: Command) {
    match cmd {
        Command::Exit => builtin::exit(),
        Command::Cd(args) => builtin::cd(&args),
        Command::Pwd => builtin::pwd(),
        Command::External(args) => match fork().expect("fork failed") {
            ForkResult::Parent { child } => {
                let _ = waitpid(child, None);
            }
            ForkResult::Child => execve_wrapper(args),
        },
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline("$ ");
        match line {
            Ok(line) => {
                let cmd = match Command::parse(&line) {
                    Some(cmd) => cmd,
                    None => continue,
                };
                execute(cmd);
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_realpath;

    #[test]
    fn test_find_realpath() {
        // found
        assert_eq!(find_realpath("sh"), "/bin/sh");

        // not found
        assert_eq!(
            find_realpath("b6f57b0a02ff43a72738a2e5be2f335690925d20cf4e89bd088d7677d7e94e99"),
            "b6f57b0a02ff43a72738a2e5be2f335690925d20cf4e89bd088d7677d7e94e99"
        );
    }
}

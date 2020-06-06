use nix::sys::wait::*;
use nix::unistd::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::ffi::CString;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum CommandKind {
    Exit,
    Cd,
    Pwd,
    External,
    None,
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    kind: CommandKind,
    args: Option<Vec<String>>,
}

impl Command {
    fn new(kind: CommandKind, args: Option<Vec<String>>) -> Self {
        Command { kind, args }
    }
}

fn get_cmd(line: String) -> Command {
    let args: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

    if args.len() < 1 {
        return Command::new(CommandKind::None, None);
    }

    let kind: CommandKind = match args[0].as_str() {
        "exit" => CommandKind::Exit,
        "cd" => CommandKind::Cd,
        "pwd" => CommandKind::Pwd,
        _ => CommandKind::External,
    };
    Command::new(kind, Some(args))
}

fn find_cmd_path(cmd_name: &str) -> String {
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

fn execve_wrapper(args: Vec<String>) {
    let path = find_cmd_path(&args[0]);
    let cpath = CString::new(path).unwrap();

    let mut cargs = Vec::<CString>::new();
    for arg in args {
        cargs.push(CString::new(arg).unwrap());
    }

    let envs: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();

    execve(&cpath, &cargs[0..], &envs).expect(&format!("failed to execute {:?}", &cargs[0]));
}

fn exec_cmd(cmd: Command) {
    match cmd.kind {
        CommandKind::Exit => {
            std::process::exit(0);
        }
        CommandKind::Cd => {
            let args = cmd.args.unwrap();
            match env::set_current_dir(&args[1]) {
                Ok(_) => {}
                Err(err) => println!("failed to change directory to '{:?}': {:?}", &args[1], err),
            }
        }
        CommandKind::Pwd => {
            let path = env::current_dir().unwrap();
            println!("{}", path.display());
        }
        CommandKind::External => match fork().expect("fork failed") {
            ForkResult::Parent { child } => {
                let _ = waitpid(child, None);
            }
            ForkResult::Child => execve_wrapper(cmd.args.unwrap()),
        },
        CommandKind::None => {}
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline("$ ");
        match line {
            Ok(line) => {
                let cmd = get_cmd(line);
                exec_cmd(cmd);
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

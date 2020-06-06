#[derive(Debug)]
pub enum Command {
    Exit,
    Pwd,
    Cd(Vec<String>),
    External(Vec<String>),
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        match args[0].as_str() {
            "exit" => Command::Exit,
            "pwd" => Command::Pwd,
            "cd" => Command::Cd(args),
            _ => Command::External(args),
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let args: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        if args.len() < 1 {
            return None;
        }
        Some(Command::new(args))
    }
}

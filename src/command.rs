#[derive(Debug, PartialEq, Eq)]
pub enum CommandKind {
    Exit,
    Cd,
    Pwd,
    External,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    pub kind: CommandKind,
    pub args: Option<Vec<String>>,
}

impl Command {
    pub fn new(kind: CommandKind, args: Option<Vec<String>>) -> Self {
        Command { kind, args }
    }
}

pub fn get_cmd(line: String) -> Command {
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

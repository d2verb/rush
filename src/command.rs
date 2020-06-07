type Arguments<'a> = Vec<&'a str>;

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Exit,
    Pwd,
    Cd(Arguments<'a>),
    External(Arguments<'a>),
}

impl<'a> Command<'a> {
    fn new(args: Vec<&'a str>) -> Self {
        match args[0] {
            "exit" => Command::Exit,
            "pwd" => Command::Pwd,
            "cd" => Command::Cd(args),
            _ => Command::External(args),
        }
    }

    pub fn parse(s: &'a str) -> Option<Self> {
        let args: Vec<&str> = s.split_whitespace().collect();
        if args.len() < 1 {
            return None;
        }
        Some(Command::new(args))
    }
}

pub struct PipedCommand<'a> {
    pub cmds: Vec<Command<'a>>,
}

impl<'a> PipedCommand<'a> {
    pub fn parse(s: &'a str) -> Option<Self> {
        let mut cmds = Vec::<Command>::new();
        for cmd_str in s.split('|').collect::<Vec<&str>>() {
            let cmd = Command::parse(cmd_str)?;
            cmds.push(cmd);
        }
        Some(Self { cmds })
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::PipedCommand;

    #[test]
    fn test_parse_command_type_correct() {
        assert_eq!(Command::parse("exit").unwrap(), Command::Exit);
        assert_eq!(Command::parse("pwd").unwrap(), Command::Pwd);
        assert!(match Command::parse("cd ..").unwrap() {
            Command::Cd(_) => true,
            _ => false,
        });
        assert!(match Command::parse("ls ..").unwrap() {
            Command::External(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_parse_command_arguments_correct() {
        let expected = vec!["echo", "a", "b", "c", "d", "efg"];
        match Command::parse("echo a b c d efg").unwrap() {
            Command::External(args) => {
                for (i, arg) in args.iter().enumerate() {
                    assert_eq!(arg, &expected[i]);
                }
            }
            c => panic!(format!("unexpected command type: {:?}", c)),
        }
    }

    #[test]
    fn test_parse_piped_command_correct_without_pipe() {
        match PipedCommand::parse("exit").unwrap() {
            PipedCommand { cmds } => {
                assert_eq!(cmds.len(), 1);
                assert_eq!(cmds[0], Command::Exit);
            }
        }
    }

    #[test]
    fn test_parse_piped_command_correct_with_pipes() {
        match PipedCommand::parse("pwd | pwd | exit").unwrap() {
            PipedCommand { cmds } => {
                assert_eq!(cmds.len(), 3);
                assert_eq!(cmds[0], Command::Pwd);
                assert_eq!(cmds[1], Command::Pwd);
                assert_eq!(cmds[2], Command::Exit);
            }
        }
    }
}

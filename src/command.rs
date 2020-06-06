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

#[cfg(test)]
mod tests {
    use super::Command;

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
}

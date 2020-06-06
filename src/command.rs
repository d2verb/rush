#[derive(Debug)]
pub enum Command<'a> {
    Exit,
    Pwd,
    Cd(Vec<&'a str>),
    External(Vec<&'a str>),
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

use std::env;

pub fn exit() {
    std::process::exit(0);
}

pub fn cd(args: &Vec<&str>) {
    match env::set_current_dir(&args[1]) {
        Ok(_) => {}
        Err(_) => println!("cd: no such directory: {}", &args[1]),
    }
}

pub fn pwd() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn test_cd() {
        cd(&vec!["cd", "/bin"]);
        assert_eq!(
            env::current_dir().unwrap().into_os_string(),
            OsString::from("/bin")
        );

        cd(&vec!["cd", "../"]);
        assert_eq!(
            env::current_dir().unwrap().into_os_string(),
            OsString::from("/")
        );
    }
}

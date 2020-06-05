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
        println!("Your input is {}", line);
    }
}

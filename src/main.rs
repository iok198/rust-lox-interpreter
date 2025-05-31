use std::io::{self, Write};
use std::process;
use std::{env, fs};

fn lines_from_prompt() -> Result<String, io::Error> {
    let mut commands = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().is_empty() {
            break;
        }

        commands.push_str(&buffer);
    }

    Ok(commands)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let lines = match &args[..] {
        [_] => lines_from_prompt()?,
        [_, file] => fs::read_to_string(file)?,
        _ => {
            eprintln!("Usage: jlox [script]");
            process::exit(64); // code 64 = EX_USAGE (command was used incorrectly)
        }
    };

    println!("{:?}", lines);

    Ok(())
}

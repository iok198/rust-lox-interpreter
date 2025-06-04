use std::io::{self, Write};
use std::{env, fs, fmt};

use rust_lox_interpreter::tokenizer::Token;
use rust_lox_interpreter::scanner::Scanner;

fn lines_from_prompt() -> Result<String, io::Error> {
    let mut commands = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush(); // allows input on same line as >
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if buffer.trim().is_empty() {
            break;
        }

        commands.push_str(&buffer);
    }

    Ok(commands)
}

// goal: LEFT_PAREN ( null
fn format_tokens(tokens: Vec<Token>) {
    for token in tokens {
        let mut name = String::new();
        let mut result = String::new();
        let _ = fmt::write(&mut name, format_args!("{:?}", token.token_type));

        for (i, c) in name.char_indices() {
            match c.is_lowercase() {
                true => result.push_str(&c.to_uppercase().to_string()),
                false => {
                    if i != 0 {result.push('_');}
                    result.push(c);
                }
            }
        }
        println!("{result} {} null", token.token_type);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => lines_from_prompt()?,
        // [_, file] => fs::read_to_string(file)?,
        [_, command, file] => {
            if command != "tokenize" { return Ok(()); }
            let contents = fs::read_to_string(file).unwrap_or(String::new());

            if !contents.is_empty() {
                let scanner = Scanner::new(contents);
                let Ok(tokens) = scanner.scan_tokens() else { panic!()};
                format_tokens(tokens);
            } else {
                println!("EOF  null");
            }

            String::new()
        }
        _ => {
            // eprintln!("Usage: jlox [script]");
            // process::exit(64); // code 64 = EX_USAGE (command was used incorrectly)
            println!("EOF  null");
            String::new()
        },
    };

    Ok(())
}

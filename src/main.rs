use std::io::{self, Write};
use std::{env, fs, fmt, process};

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
fn format_token(token: &Token) -> String {
    let mut name = String::new();
    let mut result = String::new();
    let _ = fmt::write(&mut name, format_args!("{:?}", token.token_type));

    for (i, c) in name.char_indices() {
        if c.is_lowercase() {
            result.push_str(&c.to_uppercase().to_string());
        } else {
            if i != 0 {result.push('_');}
            result.push(c);
        }
    }

    format!("{result} {} null", token.token_type)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => println!("{}", lines_from_prompt()?),
        [_, command, file] => {
            if command != "tokenize" { return Ok(()); }
            let contents = fs::read_to_string(file).unwrap_or(String::new());
            let scanner = Scanner::new(contents);
            let (tokens, errors) = scanner.scan_tokens();

            for token in tokens {
                println!("{}", format_token(&token));
            }

            if let Some(errors) = errors {
                for error in errors {
                    eprintln!("{error}");
                }

                process::exit(65);
            }
        }
        _ => {
            // eprintln!("Usage: jlox [script]");
            // process::exit(64); // code 64 = EX_USAGE (command was used incorrectly)
            println!("EOF  null");
        },
    };

    Ok(())
}

use crate::tokenizer::{Token, TokenType, TokenizerError};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
        }
    }

    fn consume_until<T, F>(iter: &mut T, callback: F) -> String
    where
        T: Iterator<Item = char>,
        F: Fn(&char) -> bool,
    {
        let mut buffer = String::new();

        for c in iter {
            if callback(&c) {
                break;
            }

            buffer.push(c);
        }
        buffer
    }

    fn scan_tokens(mut self) -> Result<Vec<Token<'a>>, TokenizerError> {
        let mut line = 1;
        let mut iter = self.source.chars().peekable();
        let mut buffer = String::new();
        let skippable = [' ', '\r', '\t', '\n'];

        while let Some(c) = iter.next() {
            if c == '\n' {
                line += 1;
            }

            if skippable.contains(&c) {
                continue;
            }

            let token_type = match (Token::from_char(c), c.is_alphabetic(), c.is_numeric()) {
                (Some(current), _, _) => {
                    if let Some(next) = iter.peek() {
                        if *next == '=' {
                            current.get_with_equals().unwrap_or(current)
                        } else {
                            current
                        }
                    } else {
                        current
                    }
                }
                (None, true, _) => {
                    let identifer = Self::consume_until(&mut iter, |ch| !ch.is_alphabetic());
                    TokenType::Identifier(identifer.as_str())
                },
                (None, _, true) => todo!("For numbers"),
                _ => return Err(TokenizerError::UnexpectedCharacter(c, line)),
            };

            self.tokens.push(Token::new(token_type, line));
        }
        self.tokens.push(Token::new(TokenType::Eof, line));
        Ok(self.tokens)
    }
}

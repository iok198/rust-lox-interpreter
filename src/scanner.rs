use crate::tokenizer::{Token, TokenType, TokenizerError};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
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

    fn scan_tokens(mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut line = 1;
        let mut iter = self.source.chars().peekable();
        let skippable = [' ', '\r', '\t', '\n'];

        while let Some(c) = iter.next() {
            if c == '\n' {
                line += 1;
            }

            if skippable.contains(&c) {
                continue;
            }

            if c.is_alphabetic() {
                let identifer = Self::consume_until(&mut iter, |ch| !ch.is_alphabetic());
                self.tokens.push(Token::new(TokenType::Identifier(identifer), line));
                continue;
            }

            if c.is_numeric() {
                let num: u32 = Self::consume_until(&mut iter, |ch| !ch.is_numeric()).parse().unwrap();

                if iter.next() != Some('.') {
                    self.tokens.push(Token::new(TokenType::Number(num, 0), line));
                    continue;
                }

                let decimal: u32 = Self::consume_until(&mut iter, |ch| !ch.is_numeric()).parse().unwrap();
                self.tokens.push(Token::new(TokenType::Number(num, decimal), line));
                continue;
            }

            let current = Token::from_char(c).ok_or(TokenizerError::UnexpectedCharacter(c, line))?;
            let Some(next) = iter.peek() else {
                self.tokens.push(Token::new(current, line));
                continue;
            };

            if *next != '=' { 
                self.tokens.push(Token::new(current, line));
                continue;
            }

            self.tokens.push(Token::new(current.get_with_equals().unwrap_or(current), line));
        }
        self.tokens.push(Token::new(TokenType::Eof, line));
        Ok(self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token};
    use super::TokenType::{self, *};

    fn get_types(tokens: Vec<Token>) -> Vec<TokenType> {
        tokens.into_iter().map(|token| token.token_type).collect()
    }

    #[test]
    fn test_single_chars() {
        let scanner = Scanner::new("()".to_owned());
        let tokens = get_types(scanner.scan_tokens().unwrap());
        assert_eq!(tokens, vec![LeftParen, RightParen, Eof]);
        let scanner = Scanner::new("{}".to_owned());
        let tokens = get_types(scanner.scan_tokens().unwrap());
        assert_eq!(tokens, vec![LeftBrace, RightBrace, Eof]);
        let scanner = Scanner::new(",.-+;/*".to_owned());
        let tokens = get_types(scanner.scan_tokens().unwrap());
        assert_eq!(tokens, vec![Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Eof]);
    }
}
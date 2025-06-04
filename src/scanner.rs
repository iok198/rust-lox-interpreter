use crate::tokenizer::{Token, TokenType, TokenizerError};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
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

    pub fn scan_tokens(mut self) -> (Vec<Token>, Option<Vec<TokenizerError>>) {
        let mut line = 1;
        let mut iter = self.source.chars().peekable();
        let mut errors: Vec<TokenizerError> = Vec::new();
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

            let current = match Token::from_char(c).ok_or(TokenizerError::UnexpectedCharacter(c, line)) {
                Ok(token) => token,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            let Some(next) = iter.peek() else {
                self.tokens.push(Token::new(current, line));
                continue;
            };

            if current == TokenType::Slash && *next == '/' {
                let _ = Self::consume_until(&mut iter, |c| *c == '\n');
                continue;
            }

            if *next != '=' { 
                self.tokens.push(Token::new(current, line));
                continue;
            }

            let token_type = match current.get_with_equals() {
                Some(token) => {
                    iter.next();
                    token
                }
                None => current
            };
            self.tokens.push(Token::new(token_type, line));
        }

        let errors = if errors.is_empty() { None } else { Some(errors) };
        self.tokens.push(Token::new(TokenType::Eof, line));
        (self.tokens, errors)
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token, TokenizerError};
    use super::TokenType::{self, *};

    fn get_types((tokens, errors): (Vec<Token>, Option<Vec<TokenizerError>>)) -> Vec<TokenType> {
        assert!(errors.is_none());
        tokens.into_iter().map(|token| token.token_type).collect()
    }

    #[test]
    fn test_single_chars() {
        let scanner = Scanner::new("()".to_owned());
        let tokens = get_types(scanner.scan_tokens());
        assert_eq!(tokens, vec![LeftParen, RightParen, Eof]);
        let scanner = Scanner::new("{}".to_owned());
        let tokens = get_types(scanner.scan_tokens());
        assert_eq!(tokens, vec![LeftBrace, RightBrace, Eof]);
        let scanner = Scanner::new(",.-+;/*".to_owned());
        let tokens = get_types(scanner.scan_tokens());
        assert_eq!(tokens, vec![Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Eof]);
    }

    #[test]
    fn test_invalid_char() {
        let scanner = Scanner::new("@".to_owned());
        let (_, errors) = scanner.scan_tokens();

        if let Some(error) = errors {
            assert_eq!(error[0], TokenizerError::UnexpectedCharacter('@', 1));
        } else {
            panic!("Didn't give an error for an invalid character");
        }
    }

    #[test]
    fn test_double_equals() {
        let scanner = Scanner::new("={===}".to_owned());
        let tokens = get_types(scanner.scan_tokens());
        assert_eq!(tokens, vec![Equal, LeftBrace, EqualEqual, Equal, RightBrace, Eof]);
    }

    #[test]
    fn test_slashes_comments() {
        let scanner = Scanner::new("()// Comment".to_owned());
        let tokens = get_types(scanner.scan_tokens());
        assert_eq!(tokens, vec![LeftParen, RightParen, Eof]);
        let scanner = Scanner::new("/()".to_owned());
        let tokens = get_types(scanner.scan_tokens());
        assert_eq!(tokens, vec![Slash, LeftParen, RightParen, Eof]);
    }
}

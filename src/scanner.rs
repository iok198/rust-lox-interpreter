use crate::tokenizer::*;
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

    fn scan_tokens(mut self) -> Vec<Token<'a>> {
        let mut line = 1;
        let mut iter = self.source.chars().peekable();
        let mut buffer = String::new();

        while let Some(c) = iter.next() {
            if c == ' ' || c == '\r' || c == '\t' {
                continue;
            }

            if c == '\n' {
                line += 1;
                continue;
            }

            if let Some(token_type) = TokenType::from_char(c) {
                self.tokens.push(Token::new(token_type, line));
                continue;
            }

            if let Some(token_type) = TokenType::from_string(&buffer) {
                self.tokens.push(Token::new(token_type, line));
                buffer = String::new();
                continue;
            }

            buffer.push(c);

            if TokenType::from_string(&buffer).is_some() {
                continue;
            }

            if let Some(next) = iter.peek() {
                if next == &'\r' || next == &'\t' || next == &' ' || next == &'\n' {
                    unimplemented!("Return error")
                }
            }
        }
        self.tokens.push(Token::new(TokenType::Eof, line));
        self.tokens
    }
}

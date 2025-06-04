use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(String),
    Number(u32, u32), // nums before and after decimal point
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl TokenType {
    pub fn get_with_equals(&self) -> Option<Self> {
        match self {
            Self::Bang => Some(Self::BangEqual),
            Self::Equal => Some(Self::EqualEqual),
            Self::Greater => Some(Self::GreaterEqual),
            Self::Less => Some(Self::LessEqual),
            _ => None,
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Minus => "-",
            Self::Plus => "+",
            Self::Semicolon => ";",
            Self::Slash => "/",
            Self::Star => "*",
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Equal => "=",
            Self::EqualEqual => "==",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Identifier(ident) => ident,
            Self::String(s) => s,
            Self::Number(num, dec) => &format!("{num}.{dec}"),
            Self::And => "and",
            Self::Class => "class",
            Self::Else => "else",
            Self::False => "false",
            Self::Fun => "fun",
            Self::For => "for",
            Self::If => "if",
            Self::Nil => "nil",
            Self::Or => "or",
            Self::Print => "print",
            Self::Return => "return",
            Self::Super => "super",
            Self::This => "this",
            Self::True => "true",
            Self::Var => "var",
            Self::While => "while",
            Self::Eof => "",
        };

        write!(f, "{display}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line: u32) -> Self {
        Self { token_type, line }
    }

    pub fn from_char(c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '/' => Some(TokenType::Slash),
            '*' => Some(TokenType::Star),
            // characters that can be appended with =
            '!' => Some(TokenType::Bang),
            '=' => Some(TokenType::Equal),
            '>' => Some(TokenType::Greater),
            '<' => Some(TokenType::Less),
            _ => None,
        }
    }

    pub fn from_string(s: &str) -> Option<TokenType> {
        match s {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.line)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum TokenizerError {
    UnexpectedCharacter(char, u32),
    UnterminatedString(u32),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter(c, line) => {
                write!(f, "[line {line}] Error: Unexpected character: {c}")
            }
            Self::UnterminatedString(line) => write!(f, "Unterminated string on line {line}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Token,
        TokenType::{self, *},
    };

    #[test]
    fn test_get_with_equals() {
        let get = TokenType::get_with_equals;
        assert_eq!(get(&Bang), Some(BangEqual));
        assert_eq!(get(&Equal), Some(EqualEqual));
        assert_eq!(get(&Greater), Some(GreaterEqual));
        assert_eq!(get(&Less), Some(LessEqual));
        assert_eq!(get(&LeftParen), None);
    }

    #[test]
    fn test_get_from_char() {
        let get = Token::from_char;
        assert_eq!(get('('), Some(LeftParen));
        assert_eq!(get(';'), Some(Semicolon));
        assert_eq!(get('%'), None);
    }

    #[test]
    fn test_get_from_string() {
        let get = Token::from_string;
        assert_eq!(get("and"), Some(And));
        assert_eq!(get("or"), Some(Or));
        assert_eq!(get("owo"), None);
    }
}

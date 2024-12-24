use crate::{
    error::InterpreterError,
    token::{Token, TokenType},
};
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    // all the source code as a peekable iterator
    source: Peekable<Chars<'a>>,
    // tokens: Vec<Token>,
    start: usize,
    pub current: usize,
    pub line: usize,
    // We're going to immediately turn sourcecode into an iterator
    source_length: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let source_length = source.len();
        Self {
            source: source.chars().peekable(),
            // tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            source_length,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.next()
    }

    fn token_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.peek() == Some(&expected) {
            self.advance();
            return true;
        }
        false
    }
    fn is_digit(c: Option<&char>) -> bool {
        if let Some(c) = c {
            (&'0'..=&'9').contains(&c)
        } else {
            false
        }
    }
    fn is_alpha(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn scan_identifier(&mut self, start_char: char) -> Result<Option<Token>, InterpreterError> {
        let mut chars = vec![start_char];
        while self.source.peek().map_or(false, Self::is_alpha) {
            chars.push(self.advance().unwrap_or_default());
        }
        let lexeme = chars.into_iter().collect::<String>();
        let token_type: TokenType =
            TokenType::scan_for_keyword(&lexeme[..]).unwrap_or(TokenType::Identifier);
        Ok(Some(Token::new(
            token_type,
            Some(lexeme),
            self.line,
            self.start,
        )))
    }
    fn scan_number(&mut self, start_char: char) -> Result<Option<Token>, InterpreterError> {
        let mut chars = vec![start_char];

        while Self::is_digit(self.source.peek()) {
            chars.push(self.advance().unwrap_or_default());
        }
        // look for a fractional part
        if self.source.peek() == Some(&'.') {
            chars.push(self.advance().unwrap_or_default());
            while Self::is_digit(self.source.peek()) {
                chars.push(self.advance().unwrap_or_default());
            }
        }
        Ok(Some(Token::new(
            TokenType::Number,
            Some(chars.into_iter().collect::<String>()),
            self.line,
            self.start,
        )))
    }

    fn scan_string(&mut self) -> Result<Option<Token>, InterpreterError> {
        let mut chars = vec![];
        while self.source.peek() != Some(&'"') && !self.is_at_end() {
            if self.source.peek() == Some(&'\n') {
                self.line += 1;
            }
            chars.push(self.advance().unwrap_or_default());
        }
        // unterminated string
        if self.is_at_end() {
            return Err(InterpreterError::ScannerError(None));
        }
        // chop the closing "
        self.advance();
        Ok(Some(Token::new(
            TokenType::String,
            Some(chars.into_iter().collect::<String>()),
            self.line,
            self.start,
        )))
    }

    fn scan_token(&mut self) -> Result<Option<Token>, InterpreterError> {
        let _char = self.advance();
        let loxchar = match _char {
            Some(loxchar) => loxchar,
            None => return Err(InterpreterError::ScannerError(None)),
        };
        let next_token = match loxchar {
            '(' => Some(Token::new(
                TokenType::LeftParen,
                None,
                self.line,
                self.start,
            )),
            ')' => Some(Token::new(
                TokenType::RightParen,
                None,
                self.line,
                self.start,
            )),
            '{' => Some(Token::new(
                TokenType::LeftBrace,
                None,
                self.line,
                self.start,
            )),
            '}' => Some(Token::new(
                TokenType::RightBrace,
                None,
                self.line,
                self.start,
            )),
            ',' => Some(Token::new(TokenType::Comma, None, self.line, self.start)),
            '.' => Some(Token::new(TokenType::Dot, None, self.line, self.start)),
            '-' => Some(Token::new(TokenType::Minus, None, self.line, self.start)),
            '+' => Some(Token::new(TokenType::Plus, None, self.line, self.start)),
            ';' => Some(Token::new(
                TokenType::Semicolon,
                None,
                self.line,
                self.start,
            )),
            '*' => Some(Token::new(TokenType::Star, None, self.line, self.start)),
            '!' => {
                if self.token_match('=') {
                    Some(Token::new(
                        TokenType::BangEqual,
                        None,
                        self.line,
                        self.start,
                    ))
                } else {
                    Some(Token::new(TokenType::Bang, None, self.line, self.start))
                }
            }
            '=' => {
                if self.token_match('=') {
                    Some(Token::new(
                        TokenType::EqualEqual,
                        None,
                        self.line,
                        self.start,
                    ))
                } else {
                    Some(Token::new(TokenType::Equal, None, self.line, self.start))
                }
            }
            '<' => {
                if self.token_match('=') {
                    Some(Token::new(
                        TokenType::LessEqual,
                        None,
                        self.line,
                        self.start,
                    ))
                } else {
                    Some(Token::new(TokenType::Less, None, self.line, self.start))
                }
            }
            '>' => {
                if self.token_match('=') {
                    Some(Token::new(
                        TokenType::GreaterEqual,
                        None,
                        self.line,
                        self.start,
                    ))
                } else {
                    Some(Token::new(TokenType::Greater, None, self.line, self.start))
                }
            }
            '/' => {
                if self.token_match('/') {
                    while self.source.peek() != Some(&'\n') && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(Token::new(TokenType::Slash, None, self.line, self.start))
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => self.scan_string()?,
            '0'..='9' => self.scan_number(loxchar)?,
            ch => {
                if Self::is_alpha(&ch) {
                    self.scan_identifier(loxchar)?
                } else {
                    return Err(InterpreterError::ScannerError(Some(loxchar.to_string())));
                }
            }
        };
        Ok(next_token)
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, InterpreterError> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            if let Some(token) = token {
                tokens.push(token);
            }
        }
        tokens.push(Token::end(self.line, self.start));
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_keywords() {
        let sources = vec![
            "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print", "return",
            "super", "this", "true", "var", "while",
        ];
        for source in sources {
            let (start, end) = source.split_at(1);
            let mut scanner = Scanner::new(end);
            let token = scanner.scan_identifier(start.chars().nth(0).expect("missing chars"));
            eprintln!("{:?}", token);
            assert_eq!(token.is_ok(), true);
            let token = token.unwrap();
            assert!(token.is_some());
            let token = token.unwrap();
            assert!(token.lexeme.is_some());
            assert!(token.token_type.is_keyword());
        }
    }

    #[test]
    fn scan_identifiers() {
        let sources = vec![
            "andifer",
            "classifier",
            "elseifer",
            "falsifier",
            "former",
            "funder",
            "if_stuff_8",
            "Nihilo",
            "orCh1d5_87_Z",
            "prints",
            "returns",
            "supers",
            "thises",
            "true_s",
            "v_a_r",
            "whilest",
        ];
        for source in sources {
            let (start, end) = source.split_at(1);
            let mut scanner = Scanner::new(end);
            let token = scanner.scan_identifier(start.chars().nth(0).expect("missing chars"));
            eprintln!("{:?}", token);
            assert_eq!(token.is_ok(), true);
            let token = token.unwrap();
            assert!(token.is_some());
            let token = token.unwrap();
            assert!(token.lexeme.is_some());
            assert!(!token.token_type.is_keyword());
        }
    }

    #[test]
    fn test_scan_number() {
        let sources: Vec<&str> = vec!["23", "23.45"];
        for source in sources {
            let mut scanner = Scanner::new(source);
            let token = scanner.scan_number('1');
            eprintln!("{:?}", token);
            assert_eq!(token.is_ok(), true);
            let token = token.unwrap();
            assert!(token.is_some());
            let token = token.unwrap();
            assert!(token.lexeme.is_some());
            assert_eq!(token.token_type, TokenType::Number);
        }
    }
    #[test]
    fn test_scan_string() {
        // we assume a string passed to this method will have no opening "
        let source = "hello world\"";
        let mut scanner = Scanner::new(source);
        let token = scanner.scan_string();
        eprintln!("{:?}", token);
        assert_eq!(token.is_ok(), true);
        let token = token.unwrap();
        assert!(token.is_some());
        let token = token.unwrap();
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(token.lexeme, Some("hello world".to_string()));
    }

    #[test]
    fn test_scan_tokens_string() {
        let source = "\"hello world\"";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_scan_various() {
        let cases = vec![
            "// this is a comment",
            "(( )){} // grouping stuff",
            "!*+-/=<> <= == // operators",
        ];
        for case in cases {
            let mut scanner = Scanner::new(case);
            assert!(scanner.scan_tokens().is_ok());
        }
    }

    #[test]
    fn test_scan_tokens() {
        let source = "var a = 1;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
        assert_eq!(tokens[5].token_type, TokenType::Eof);
    }
}

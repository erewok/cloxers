use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
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
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
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
    // error
    TokenError,
    // End of file.
    Eof,
}

impl TokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::And
                | TokenType::Class
                | TokenType::Else
                | TokenType::False
                | TokenType::Fun
                | TokenType::For
                | TokenType::If
                | TokenType::Nil
                | TokenType::Or
                | TokenType::Print
                | TokenType::Return
                | TokenType::Super
                | TokenType::This
                | TokenType::True
                | TokenType::Var
                | TokenType::While
        )
    }
    /// If the lexeme is a keyword, return the keyword token type
    pub fn scan_for_keyword(lexeme: &str) -> Option<TokenType> {
        let result = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => return None,
        };
        Some(result)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",
            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",
            TokenType::And => "And",
            TokenType::Class => "Class",
            TokenType::Else => "Else",
            TokenType::False => "False",
            TokenType::Fun => "Fun",
            TokenType::For => "For",
            TokenType::If => "If",
            TokenType::Nil => "Nil",
            TokenType::Or => "Or",
            TokenType::Print => "Print",
            TokenType::Return => "Return",
            TokenType::Super => "Super",
            TokenType::This => "This",
            TokenType::True => "True",
            TokenType::Var => "Var",
            TokenType::While => "While",
            TokenType::TokenError => "TokenError",
            TokenType::Eof => "Eof",
        };
        write!(f, "{}", token)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    // a lexeme is not always necessary to get the drift
    pub lexeme: Option<String>,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }

    pub fn end(line: usize, column: usize) -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: None,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}::{}] {:?} => '{}'",
            self.line,
            self.column,
            self.token_type,
            self.lexeme.as_deref().unwrap_or_default()
        )
    }
}

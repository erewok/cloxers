use crate::token::TokenType;

/// Pratt Parser Parsing Rules
/// Instead of pointers to functions, we'll
/// use an enum to represent the strategy.
pub struct ParseRule {
    pub kind: TokenType,
    pub prefix: Option<ParseRuleStrategy>,
    pub infix: Option<ParseRuleStrategy>,
    pub precedence: ParsePrecedence,
}

const RULES: [ParseRule; 40] = [
    ParseRule {kind: TokenType::LeftParen, prefix: Some(ParseRuleStrategy::Grouping), infix: Some(ParseRuleStrategy::Call), precedence: ParsePrecedence::Call},
    ParseRule {kind: TokenType::RightParen, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::LeftBrace, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::RightBrace, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Comma, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Dot, prefix: None, infix: Some(ParseRuleStrategy::Dot), precedence: ParsePrecedence::Call},
    ParseRule {kind: TokenType::Minus, prefix: Some(ParseRuleStrategy::Unary), infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Term},
    ParseRule {kind: TokenType::Plus, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Term},
    ParseRule {kind: TokenType::Semicolon, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Slash, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Factor},
    ParseRule {kind: TokenType::Star, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Factor},
    ParseRule {kind: TokenType::Bang, prefix: Some(ParseRuleStrategy::Unary), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::BangEqual, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Equality},
    ParseRule {kind: TokenType::Equal, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::EqualEqual, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Equality},
    ParseRule {kind: TokenType::Greater, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Comparison},
    ParseRule {kind: TokenType::GreaterEqual, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Comparison},
    ParseRule {kind: TokenType::Less, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Comparison},
    ParseRule {kind: TokenType::LessEqual, prefix: None, infix: Some(ParseRuleStrategy::Binary), precedence: ParsePrecedence::Comparison},
    ParseRule {kind: TokenType::Identifier, prefix: Some(ParseRuleStrategy::Variable), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::String, prefix: Some(ParseRuleStrategy::String), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Number, prefix: Some(ParseRuleStrategy::Number), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::And, prefix: None, infix: Some(ParseRuleStrategy::And), precedence: ParsePrecedence::And},
    ParseRule {kind: TokenType::Class, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Else, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::False, prefix: Some(ParseRuleStrategy::Literal), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::For, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Fun, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::If, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Nil, prefix: Some(ParseRuleStrategy::Literal), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Or, prefix: None, infix: Some(ParseRuleStrategy::Or), precedence: ParsePrecedence::Or},
    ParseRule {kind: TokenType::Print, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Return, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Super, prefix: Some(ParseRuleStrategy::Super), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::This, prefix: Some(ParseRuleStrategy::This), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::True, prefix: Some(ParseRuleStrategy::Literal), infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Var, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::While, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::TokenError, prefix: None, infix: None, precedence: ParsePrecedence::None},
    ParseRule {kind: TokenType::Eof, prefix: None, infix: None, precedence: ParsePrecedence::None},
];


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ParsePrecedence {
    None = 1,
    Assignment = 2,
    Or = 3,
    And = 4,
    Equality = 5,
    Comparison = 6,
    Term = 7,
    Factor = 8,
    Unary = 9,
    Call = 10,
    Primary = 11,
}

impl Ord for ParsePrecedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}
impl PartialOrd for ParsePrecedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}

/// Parse rule strategy: each matches a compiler function
#[derive(Debug, Clone, PartialEq)]
pub enum ParseRuleStrategy {
    Grouping,
    Call,
    Dot,
    Binary,
    Unary,
    Variable,
    String,
    Number,
    And,
    Or,
    Literal,
    Super,
    This,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_parse_rule() {
        let rule = ParseRule {
            kind: TokenType::LeftParen,
            prefix: Some(ParseRuleStrategy::Grouping),
            infix: Some(ParseRuleStrategy::Call),
            precedence: ParsePrecedence::Call,
        };
        assert_eq!(rule.kind, TokenType::LeftParen);
        assert_eq!(rule.prefix, Some(ParseRuleStrategy::Grouping));
        assert_eq!(rule.infix, Some(ParseRuleStrategy::Call));
        assert_eq!(rule.precedence, ParsePrecedence::Call);
    }

    #[test]
    fn test_parse_rule_strategy() {
        let strategy = ParseRuleStrategy::Grouping;
        assert_eq!(strategy, ParseRuleStrategy::Grouping);
    }

    #[test]
    fn test_parse_precedence() {
        let precedence = ParsePrecedence::None;
        assert_eq!(precedence, ParsePrecedence::None);
        assert!(ParsePrecedence::None < ParsePrecedence::Assignment);
        assert!(ParsePrecedence::Assignment < ParsePrecedence::Or);
        assert!(ParsePrecedence::Or < ParsePrecedence::And);
        assert!(ParsePrecedence::And < ParsePrecedence::Equality);
        assert!(ParsePrecedence::Equality < ParsePrecedence::Comparison);
        assert!(ParsePrecedence::Comparison < ParsePrecedence::Term);
    }

    /// Sanity check: if the order of the TokenType enum changes, this test will fail.
    #[test]
    fn test_rule_token_type_order_consistency() {
        for kind in TokenType::iter() {
            let rule = &RULES[kind.clone() as usize];
            assert_eq!(rule.kind, kind);
        }
    }
}
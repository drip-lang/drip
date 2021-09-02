use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum TokenKind {
    // remove those
    Root,
    InfixExpression,
    PrefixExpression,

    #[error]
    Error,
    #[regex("[ \n]+")]
    Whitespace,
    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,
    #[token("let")]
    LetKw,
    #[token("fn")]
    FnKw,

    #[regex("[0-9]+", priority = 2)]
    Integer,
    #[regex("([0-9]*[.])?[0-9]+")]
    Float,

    #[token("!")]
    Bang,
    #[token("?")]
    Quot,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("^")]
    Circumflex,
    #[token("^^")]
    Circumflex2,

    #[token("&")]
    And,
    #[token("&&")]
    And2,
    #[token("|")]
    Pipe,
    #[token("||")]
    Pipe2,

    #[token("=")]
    Equals,
    #[token("==")]
    Equals2,
    #[token("===")]
    Equals3,

    #[token("%")]
    Percent,
    #[token("$")]
    Dollar,
    #[token("#")]
    Hashtag,
    #[token("@")]
    At,
    #[token("_")]
    Underscore,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("\"")]
    Quote,
    #[token("\'")]
    SingleQuote,

    // brackets
    #[token("(")]
    LRoundBracket,
    #[token(")")]
    RRoundBracket,
    #[token("<")]
    LAngledBracket,
    #[token("<<")]
    LAngledBracket2,
    #[token(">")]
    RAngledBracket,
    #[token(">>")]
    RAngledBracket2,
    #[token("[")]
    LSquareBracket,
    #[token("]")]
    RSquareBracket,
    #[token("{")]
    LCurlyBracket,
    #[token("}")]
    RCurlyBracket,
}

pub struct Lexer<'a> {
    pub inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (TokenKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}

impl From<TokenKind> for rowan::SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        Self(kind as u16)
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    fn assert(input: &str, token: TokenKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((token, input)));
    }

    #[test]
    fn spaces() {
        assert(" ", TokenKind::Whitespace);
        assert("   ", TokenKind::Whitespace);
    }

    #[test]
    fn fn_keyword() {
        assert("fn", TokenKind::FnKw);
    }

    #[test]
    fn let_keyword() {
        assert("let", TokenKind::LetKw);
    }

    #[test]
    fn ident() {
        assert("Aksdf123", TokenKind::Ident);
        assert("SDFSLS3234DSFD", TokenKind::Ident);
    }

    #[test]
    fn integer() {
        assert("123", TokenKind::Integer);
        assert("41242343", TokenKind::Integer);
    }

    #[test]
    fn float() {
        assert(".13", TokenKind::Float);
        assert("2535.324", TokenKind::Float);
    }

    #[test]
    fn plus() {
        assert("+", TokenKind::Plus);
    }

    #[test]
    fn minus() {
        assert("-", TokenKind::Minus);
    }

    #[test]
    fn star() {
        assert("*", TokenKind::Star);
    }

    #[test]
    fn slash() {
        assert("/", TokenKind::Slash);
    }

    #[test]
    fn circumflex() {
        assert("^", TokenKind::Circumflex);
    }

    #[test]
    fn double_circumflex() {
        assert("^^", TokenKind::Circumflex2);
    }

    #[test]
    fn equals() {
        assert("=", TokenKind::Equals);
    }

    #[test]
    fn l_brace() {
        assert("{", TokenKind::LCurlyBracket);
    }

    #[test]
    fn r_brace() {
        assert("}", TokenKind::RCurlyBracket);
    }

    #[test]
    fn errors() {
        assert("¤", TokenKind::Error);
    }

    #[test]
    fn expression() {
        let input = "5+8*3-2";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((TokenKind::Integer, "5")));
        assert_eq!(lexer.next(), Some((TokenKind::Plus, "+")));
        assert_eq!(lexer.next(), Some((TokenKind::Integer, "8")));
        assert_eq!(lexer.next(), Some((TokenKind::Star, "*")));
        assert_eq!(lexer.next(), Some((TokenKind::Integer, "3")));
        assert_eq!(lexer.next(), Some((TokenKind::Minus, "-")));
        assert_eq!(lexer.next(), Some((TokenKind::Integer, "2")));
    }
}

use logos::Logos;
use std::fmt::{self, Formatter};

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    #[error]
    Error,

    #[regex("//.*")]
    Comment,
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
    Quest,
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

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            TokenKind::Error => "unrecognized token",
            TokenKind::Comment => "comment",
            TokenKind::Whitespace => "whitespace",
            TokenKind::Ident => "identifier",
            TokenKind::LetKw => "'let'",
            TokenKind::FnKw => "'fn'",
            TokenKind::Integer => "default integer",
            TokenKind::Float => "default float",
            TokenKind::Bang => "!",
            TokenKind::Quest => "'?'",
            TokenKind::Plus => "'+'",
            TokenKind::Minus => "'-'",
            TokenKind::Star => "'*'",
            TokenKind::Slash => "'/'",
            TokenKind::Circumflex => "'^'",
            TokenKind::Circumflex2 => "'^^'",
            TokenKind::And => "'&'",
            TokenKind::And2 => "'&&'",
            TokenKind::Pipe => "'|'",
            TokenKind::Pipe2 => "'||'",
            TokenKind::Equals => "'='",
            TokenKind::Equals2 => "'=='",
            TokenKind::Equals3 => "'==='",
            TokenKind::Percent => "'%'",
            TokenKind::Dollar => "'$'",
            TokenKind::Hashtag => "'#'",
            TokenKind::At => "'@'",
            TokenKind::Underscore => "'_'",
            TokenKind::Dot => "'.'",
            TokenKind::Comma => "','",
            TokenKind::Colon => "':'",
            TokenKind::Semicolon => "';'",
            TokenKind::Quote => "'\"'",
            TokenKind::SingleQuote => "'''",
            TokenKind::LRoundBracket => "'('",
            TokenKind::RRoundBracket => "')'",
            TokenKind::LAngledBracket => "'<'",
            TokenKind::LAngledBracket2 => "'<<'",
            TokenKind::RAngledBracket => "'>'",
            TokenKind::RAngledBracket2 => "'>>'",
            TokenKind::LSquareBracket => "'['",
            TokenKind::RSquareBracket => "']'",
            TokenKind::LCurlyBracket => "'{'",
            TokenKind::RCurlyBracket => "'}'",
        })
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::*;

    fn assert(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
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
}

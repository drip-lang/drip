use drip_lexer::TokenKind;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum SyntaxKind {
    Root,
    Error,
    Comment,
    Literal,
    InfixExpr,
    PrefixExpr,
    RoundBracketExpr,
    VariableDef,

    // Lang
    Whitespace,
    Ident,
    LetKw,
    FnKw,

    Integer,
    Float,

    Bang,
    Quest,

    Plus,
    Minus,
    Star,
    Slash,
    Circumflex,
    Circumflex2,

    And,
    And2,

    Pipe,
    Pipe2,

    Equals,
    Equals2,
    Equals3,

    Percent,
    Dollar,
    Hashtag,
    At,
    Underscore,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Quote,
    SingleQuote,

    LRoundBracket,
    RRoundBracket,

    LAngledBracket,
    RAngledBracket,

    LAngledBracket2,
    RAngledBracket2,

    LSquareBracket,
    RSquareBracket,

    LCurlyBracket,
    RCurlyBracket,
}

pub type SyntaxNode = rowan::SyntaxNode<Drip>;
pub type SyntaxElement = rowan::SyntaxElement<Drip>;
pub type SyntaxToken = rowan::SyntaxToken<Drip>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Drip {}

impl rowan::Language for Drip {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::try_from(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token: TokenKind) -> Self {
        match token {
            TokenKind::Error => SyntaxKind::Error,
            TokenKind::Whitespace => SyntaxKind::Whitespace,
            TokenKind::Ident => SyntaxKind::Ident,
            TokenKind::LetKw => SyntaxKind::LetKw,
            TokenKind::FnKw => SyntaxKind::FnKw,
            TokenKind::Integer => SyntaxKind::Integer,
            TokenKind::Float => SyntaxKind::Float,
            TokenKind::Bang => SyntaxKind::Bang,
            TokenKind::Quest => SyntaxKind::Quest,
            TokenKind::Plus => SyntaxKind::Plus,
            TokenKind::Minus => SyntaxKind::Minus,
            TokenKind::Star => SyntaxKind::Star,
            TokenKind::Slash => SyntaxKind::Slash,
            TokenKind::Circumflex => SyntaxKind::Circumflex,
            TokenKind::Circumflex2 => SyntaxKind::Circumflex2,
            TokenKind::And => SyntaxKind::And,
            TokenKind::And2 => SyntaxKind::And2,
            TokenKind::Pipe => SyntaxKind::Pipe,
            TokenKind::Pipe2 => SyntaxKind::Pipe2,
            TokenKind::Equals => SyntaxKind::Equals,
            TokenKind::Equals2 => SyntaxKind::Equals2,
            TokenKind::Equals3 => SyntaxKind::Equals3,
            TokenKind::Percent => SyntaxKind::Percent,
            TokenKind::Dollar => SyntaxKind::Dollar,
            TokenKind::Hashtag => SyntaxKind::Hashtag,
            TokenKind::At => SyntaxKind::At,
            TokenKind::Underscore => SyntaxKind::Underscore,
            TokenKind::Dot => SyntaxKind::Dot,
            TokenKind::Comma => SyntaxKind::Comma,
            TokenKind::Colon => SyntaxKind::Colon,
            TokenKind::Semicolon => SyntaxKind::Semicolon,
            TokenKind::Quote => SyntaxKind::Quote,
            TokenKind::SingleQuote => SyntaxKind::SingleQuote,
            TokenKind::LRoundBracket => SyntaxKind::LRoundBracket,
            TokenKind::RRoundBracket => SyntaxKind::LRoundBracket,
            TokenKind::LAngledBracket => SyntaxKind::RRoundBracket,
            TokenKind::LAngledBracket2 => SyntaxKind::LAngledBracket2,
            TokenKind::RAngledBracket => SyntaxKind::LAngledBracket,
            TokenKind::RAngledBracket2 => SyntaxKind::RAngledBracket2,
            TokenKind::LSquareBracket => SyntaxKind::RAngledBracket,
            TokenKind::RSquareBracket => SyntaxKind::LSquareBracket,
            TokenKind::LCurlyBracket => SyntaxKind::RSquareBracket,
            TokenKind::RCurlyBracket => SyntaxKind::LCurlyBracket,
            TokenKind::Comment => SyntaxKind::Comment,
        }
    }
}

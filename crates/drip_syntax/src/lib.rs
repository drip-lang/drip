use drip_lexer::TokenKind;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum SyntaxKind {
    Root,
    Error,
    Comment,
    Whitespace,

    InfixExpr,
    PrefixExpr,

    RoundBracketExpr,
    BlockExpr,

    ConstDef,
    VariableDef,
    VariableRef,
    AssignDef,

    FnDef,
    FnParamListDef,
    FnParamDef,

    FnReturnDef,
    FnReturnTypeListDef,
    FnReturnTypeDef,

    FnBodyDef,

    StructDef,
    StructFieldListDef,
    StructFieldDef,

    TraitDef,
    TraitListsDef,
    TraitTypeListDef,
    TraitTypeDef,
    TraitFnListDef,

    FnKw,
    StructKw,
    TraitKw,
    Ident,
    Type,
    TypeKw,
    ConstKw,
    VariableKw,
    SelfVarKw,
    SelfTypeKw,

    Literal,
    Number,

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
    Grave,
    Arrow,

    LRoundBracket,
    RRoundBracket,

    LAngledBracket,
    RAngledBracket,

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
            TokenKind::FnKw => SyntaxKind::FnKw,
            TokenKind::StructKw => SyntaxKind::StructKw,
            TokenKind::TraitKw => SyntaxKind::TraitKw,
            TokenKind::ConstKw => SyntaxKind::ConstKw,
            TokenKind::VariableKw => SyntaxKind::VariableKw,
            TokenKind::TypeKw => SyntaxKind::TypeKw,
            TokenKind::SelfVarKw => SyntaxKind::SelfVarKw,
            TokenKind::SelfTypeKw => SyntaxKind::SelfTypeKw,
            TokenKind::Ident => SyntaxKind::Ident,
            TokenKind::Number => SyntaxKind::Number,
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
            TokenKind::Grave => SyntaxKind::Grave,
            TokenKind::Arrow => SyntaxKind::Arrow,
            TokenKind::LRoundBracket => SyntaxKind::LRoundBracket,
            TokenKind::RRoundBracket => SyntaxKind::RRoundBracket,
            TokenKind::LAngledBracket => SyntaxKind::LAngledBracket,
            TokenKind::RAngledBracket => SyntaxKind::RAngledBracket,
            TokenKind::LSquareBracket => SyntaxKind::LSquareBracket,
            TokenKind::RSquareBracket => SyntaxKind::RSquareBracket,
            TokenKind::LCurlyBracket => SyntaxKind::LCurlyBracket,
            TokenKind::RCurlyBracket => SyntaxKind::RCurlyBracket,
            TokenKind::Comment => SyntaxKind::Comment,
        }
    }
}

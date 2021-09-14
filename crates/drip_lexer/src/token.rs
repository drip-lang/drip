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
    #[regex("[/\\p{L}/u][/\\p{L}/u-z0-9_]*")]
    Ident,
    #[token("fn")]
    FnKw,

    #[token("use")]
    UseKw,
    #[token("extern")]
    ExternKw,

    #[token("struct")]
    StructKw,
    #[token("trait")]
    TraitKw,
    #[token("::")]
    ConstKw,
    #[token(":=")]
    VariableKw,

    #[token("Type")]
    TypeKw,
    #[token("self")]
    SelfVarKw,
    #[token("Self")]
    SelfTypeKw,

    #[regex("([0-9][0-9_]*)?\\.?[0-9_]+([eE][-+]?[0-9_]+)?")]
    Number,

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
    #[token("`")]
    Grave,
    #[token("->")]
    Arrow,
    // brackets
    #[token("{")]
    LCurlyBracket,
    #[token("}")]
    RCurlyBracket,
    #[token("(")]
    LRoundBracket,
    #[token(")")]
    RRoundBracket,
    #[token("<")]
    LAngledBracket,
    #[token(">")]
    RAngledBracket,
    #[token("[")]
    LSquareBracket,
    #[token("]")]
    RSquareBracket,
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
            TokenKind::FnKw => "fn",
            TokenKind::StructKw => "struct",
            TokenKind::TraitKw => "trait",
            TokenKind::Ident => "identifier",
            TokenKind::TypeKw => "type",
            TokenKind::SelfVarKw => "'self'",
            TokenKind::SelfTypeKw => "'Self'",
            TokenKind::ConstKw => "::",
            TokenKind::VariableKw => ":=",
            TokenKind::Number => "number",
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
            TokenKind::Grave => "'`'",
            TokenKind::Arrow => "->",
            TokenKind::LCurlyBracket => "'{'",
            TokenKind::RCurlyBracket => "'}'",
            TokenKind::LRoundBracket => "'('",
            TokenKind::RRoundBracket => "')'",
            TokenKind::LAngledBracket => "'<'",
            TokenKind::RAngledBracket => "'>'",
            TokenKind::LSquareBracket => "'['",
            TokenKind::RSquareBracket => "']'",
            TokenKind::UseKw => "use",
            TokenKind::ExternKw => "extern",
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
    fn comments() {
        assert("//This is a comment", TokenKind::Comment);
    }

    #[test]
    fn whitespaces() {
        assert(" ", TokenKind::Whitespace);
        assert("   ", TokenKind::Whitespace);
        assert("\n", TokenKind::Whitespace);
        assert("    \n ", TokenKind::Whitespace);
    }

    #[test]
    fn ident() {
        assert("Aksdf123", TokenKind::Ident);
        assert("SDFSLS3234DSFD", TokenKind::Ident);
        assert("안녕하세요", TokenKind::Ident);
        assert("안녕하세요3", TokenKind::Ident);
        assert("Gerät3是", TokenKind::Ident);
    }

    #[test]
    fn struct_kw() {
        assert("struct", TokenKind::StructKw);
    }

    #[test]
    fn trait_kw() {
        assert("trait", TokenKind::TraitKw);
    }

    #[test]
    fn const_kw() {
        assert("::", TokenKind::ConstKw);
    }

    #[test]
    fn variable_kw() {
        assert(":=", TokenKind::VariableKw);
    }

    #[test]
    fn self_var_kw() {
        assert("self", TokenKind::SelfVarKw);
    }

    #[test]
    fn self_type_kw() {
        assert("Self", TokenKind::SelfTypeKw);
    }

    #[test]
    fn number() {
        assert("123", TokenKind::Number);
        assert("41242343", TokenKind::Number);
        assert(".13", TokenKind::Number);
        assert("2535.324", TokenKind::Number);
        assert("2535.324e10", TokenKind::Number);
        assert("23_629_349_234", TokenKind::Number);
        assert("2_____4", TokenKind::Number);
        assert("2_490.0", TokenKind::Number);
        assert("2_490.023_423_7", TokenKind::Number);
    }

    #[test]
    fn bang() {
        assert("!", TokenKind::Bang);
    }

    #[test]
    fn quest() {
        assert("?", TokenKind::Quest);
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
    fn circumflex2() {
        assert("^^", TokenKind::Circumflex2);
    }

    #[test]
    fn and() {
        assert("&", TokenKind::And);
    }

    #[test]
    fn and2() {
        assert("&&", TokenKind::And2);
    }

    #[test]
    fn pipe() {
        assert("|", TokenKind::Pipe);
    }

    #[test]
    fn pipe2() {
        assert("||", TokenKind::Pipe2);
    }

    #[test]
    fn equals() {
        assert("=", TokenKind::Equals);
    }

    #[test]
    fn equals2() {
        assert("==", TokenKind::Equals2);
    }

    #[test]
    fn percent() {
        assert("%", TokenKind::Percent);
    }

    #[test]
    fn dollar() {
        assert("$", TokenKind::Dollar);
    }

    #[test]
    fn hashtag() {
        assert("#", TokenKind::Hashtag);
    }

    #[test]
    fn at() {
        assert("@", TokenKind::At);
    }

    #[test]
    fn underscore() {
        assert("_", TokenKind::Underscore);
    }

    #[test]
    fn dot() {
        assert(".", TokenKind::Dot);
    }

    #[test]
    fn comma() {
        assert(",", TokenKind::Comma);
    }

    #[test]
    fn colon() {
        assert(":", TokenKind::Colon);
    }

    #[test]
    fn semicolon() {
        assert(";", TokenKind::Semicolon);
    }

    #[test]
    fn quote() {
        assert("\"", TokenKind::Quote);
    }

    #[test]
    fn single_quote() {
        assert("\'", TokenKind::SingleQuote);
    }

    #[test]
    fn grave() {
        assert("`", TokenKind::Grave);
    }

    #[test]
    fn l_round_bracket() {
        assert("(", TokenKind::LRoundBracket);
    }

    #[test]
    fn r_round_bracket() {
        assert(")", TokenKind::RRoundBracket);
    }

    #[test]
    fn l_angled_bracket() {
        assert("<", TokenKind::LAngledBracket);
    }

    #[test]
    fn r_angled_bracket() {
        assert(">", TokenKind::RAngledBracket);
    }

    #[test]
    fn l_square_bracket() {
        assert("[", TokenKind::LSquareBracket);
    }

    #[test]
    fn r_square_bracket() {
        assert("]", TokenKind::RSquareBracket);
    }

    #[test]
    fn l_curly_bracket() {
        assert("{", TokenKind::LCurlyBracket);
    }

    #[test]
    fn r_curly_bracket() {
        assert("}", TokenKind::RCurlyBracket);
    }

    #[test]
    fn errors() {
        assert("¤", TokenKind::Error);
    }
}

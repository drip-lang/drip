use crate::parser::Parser;
use drip_lexer::TokenKind;
use drip_syntax::SyntaxKind;

pub(super) fn types(p: &mut Parser) {
    match p.current() {
        Some(token) => {
            // TODO: extend this to include Type paths once a module / package system is decided
            if is_path_type_start(token) {
                let m = p.start();
                p.bump();
                m.complete(p, SyntaxKind::Type);
            }
        }
        None => {
            unreachable!()
        }
    }
}

// TODO: add "package / module path", after module system decided
fn is_path_type_start(token: TokenKind) -> bool {
    matches!(token, TokenKind::Ident | TokenKind::SelfTypeKw)
}

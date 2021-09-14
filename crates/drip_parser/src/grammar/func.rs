use crate::grammar::{decl, types};
use crate::parser::Parser;
use drip_lexer::TokenKind;
use drip_syntax::SyntaxKind;

pub(crate) fn function_def(p: &mut Parser) {
    p.expect(TokenKind::LRoundBracket);
    if !p.at(TokenKind::RRoundBracket) {
        function_param_list_def(p);
    }
    p.expect(TokenKind::RRoundBracket);
    // return is optional
    if p.at(TokenKind::Arrow) {
        p.bump();
        function_return_def(p);
    }
    p.expect(TokenKind::LCurlyBracket);
    if !p.at(TokenKind::RCurlyBracket) {
        function_body_def(p);
    }
    p.expect(TokenKind::RCurlyBracket);
}

fn function_param_list_def(p: &mut Parser) {
    let m = p.start();
    while !p.at(TokenKind::RRoundBracket) {
        function_param_def(p);
    }
    m.complete(p, SyntaxKind::FnParamListDef);
}

fn function_body_def(p: &mut Parser) {
    let m = p.start();
    decl::decl(p);
    m.complete(p, SyntaxKind::FnBodyDef);
}

fn function_return_def(p: &mut Parser) {
    let m = p.start();
    if p.at(TokenKind::LRoundBracket) {
        p.bump();
        function_return_type_list_def(p);
        p.expect(TokenKind::RRoundBracket)
    } else {
        function_return_type_def(p);
    }
    m.complete(p, SyntaxKind::FnReturnDef);
}

fn function_return_type_list_def(p: &mut Parser) {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    while !p.at(TokenKind::RRoundBracket) {
        function_return_type_def(p);
        if p.at(TokenKind::Comma) {
            p.bump();
        }
    }
    m.complete(p, SyntaxKind::FnReturnTypeListDef);
}

fn function_return_type_def(p: &mut Parser) {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    types::types(p);
    m.complete(p, SyntaxKind::FnReturnTypeDef);
}

fn function_param_def(p: &mut Parser) {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    p.bump();
    p.expect(TokenKind::Colon);
    types::types(p);
    // `,` is optional after parameter
    if p.at(TokenKind::Comma) {
        p.bump()
    }
    m.complete(p, SyntaxKind::FnParamDef);
}

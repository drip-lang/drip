use crate::grammar::decl;
use crate::grammar::types;
use crate::parser::Parser;
use drip_lexer::TokenKind;
use drip_syntax::SyntaxKind;

pub(crate) fn struct_def(p: &mut Parser) {
    assert!(p.at(TokenKind::StructKw));
    p.bump();
    struct_field_list_def(p);
}

fn struct_field_list_def(p: &mut Parser) {
    assert!(p.at(TokenKind::LCurlyBracket));
    let m = p.start();
    p.bump();
    while !p.at(TokenKind::RCurlyBracket) {
        // TODO: handle "{ a: }", "{ a: {",  "{ {"
        struct_field_def(p);
    }
    p.expect(TokenKind::RCurlyBracket);
    m.complete(p, SyntaxKind::StructFieldListDef);
}

fn struct_field_def(p: &mut Parser) {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    p.bump();
    p.expect(TokenKind::Colon);
    types::types(p);
    // `,` is optional after fields
    if p.at(TokenKind::Comma) {
        p.bump()
    }
    m.complete(p, SyntaxKind::StructFieldDef);
}

pub(crate) fn trait_def(p: &mut Parser) {
    assert!(p.at(TokenKind::TraitKw));
    p.bump();
    trait_lists_def(p);
}

fn trait_lists_def(p: &mut Parser) {
    assert!(p.at(TokenKind::LCurlyBracket));
    let m = p.start();
    p.bump();
    if !p.at(TokenKind::RCurlyBracket) {
        if p.at(TokenKind::TypeKw) {
            trait_type_lists_def(p);
            unimplemented!();
        } else {
            trait_fn_list_def(p);
        }
    }
    p.expect(TokenKind::RCurlyBracket);
    m.complete(p, SyntaxKind::TraitListsDef);
}

fn trait_type_lists_def(p: &mut Parser) {
    assert!(p.at(TokenKind::TypeKw));
    let m = p.start();
    m.complete(p, SyntaxKind::TraitTypeListDef);
}

fn trait_fn_list_def(p: &mut Parser) {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();
    decl::decl(p);
    m.complete(p, SyntaxKind::TraitFnListDef);
}

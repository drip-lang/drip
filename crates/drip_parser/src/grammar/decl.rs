use crate::grammar::{expr, func, structs};
use crate::marker::Marker;
use crate::parser::Parser;
use drip_lexer::TokenKind;
use drip_syntax::SyntaxKind;

pub(crate) fn decl(p: &mut Parser) {
    if p.at(TokenKind::ExternKw) {
        abi(p);
    } else if p.at(TokenKind::UseKw) {
        import(p);
    } else if p.at(TokenKind::Ident) {
        ident_decl(p);
    } else {
        expr::expr(p);
    }
}

fn ident_decl(p: &mut Parser) {
    assert!(p.at(TokenKind::Ident));
    let m = p.start();

    let peek = p.peek_nth(1);
    if peek.is_some() {
        let peek = peek.unwrap();
        if peek == TokenKind::Equals {
            let m_ref = p.start();
            p.bump();
            m_ref.complete(p, SyntaxKind::VariableRef);
            assign_def(p);
            m.complete(p, SyntaxKind::AssignDef);
            return;
        }
        if peek != TokenKind::ConstKw && peek != TokenKind::VariableKw {
            expr::expr(p);
            m.abandon(p);
            return;
        }
    }

    if peek.is_none() {
        expr::expr(p);
        m.abandon(p);
        return;
    }

    p.bump();

    if p.at(TokenKind::ConstKw) {
        const_def(p, m);
    } else if p.at(TokenKind::VariableKw) {
        variable_def(p);
        m.complete(p, SyntaxKind::VariableDef);
    } else {
        unreachable!();
    }
}

fn const_def(p: &mut Parser, m: Marker) {
    assert!(p.at(TokenKind::ConstKw));
    p.bump();

    if p.at(TokenKind::StructKw) {
        structs::struct_def(p);
        m.complete(p, SyntaxKind::StructDef);
    } else if p.at(TokenKind::TraitKw) {
        structs::trait_def(p);
        m.complete(p, SyntaxKind::TraitDef);
    } else {
        if p.current().unwrap() == TokenKind::LRoundBracket {
            let mut counter = 0;
            while p.peek_nth(counter).unwrap() != TokenKind::RRoundBracket {
                counter += 1;
            }
            let nth = p.peek_nth(counter + 1).unwrap();
            if nth == TokenKind::Arrow || nth == TokenKind::LCurlyBracket {
                func::function_def(p);
                m.complete(p, SyntaxKind::FnDef);
            } else {
                expr::expr(p);
                m.complete(p, SyntaxKind::ConstDef);
            }
        } else {
            expr::expr(p);
            m.complete(p, SyntaxKind::ConstDef);
        }
    }
}

fn variable_def(p: &mut Parser) {
    assert!(p.at(TokenKind::VariableKw));
    p.bump();
    expr::expr(p);
}

fn assign_def(p: &mut Parser) {
    assert!(p.at(TokenKind::Equals));
    p.bump();
    expr::expr(p);
}

fn abi(_p: &mut Parser) {
    unimplemented!();
}

fn import(_p: &mut Parser) {
    unimplemented!();
}

use crate::lexer::TokenKind;
use crate::parser::Parser;

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

pub fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

pub fn expr_binding_power(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(TokenKind::Integer) | Some(TokenKind::Ident) => p.bump(),
        Some(TokenKind::Minus) => {
            let op = PrefixOp::Neg;
            let ((), right_binding_power) = op.binding_power();

            p.bump();

            p.start_node_at(checkpoint, TokenKind::PrefixExpression);
            expr_binding_power(p, right_binding_power);
            p.finish_node();
        }
        Some(TokenKind::LRoundBracket) => {
            p.bump();
            expr_binding_power(p, 0);
            assert_eq!(p.peek(), Some(TokenKind::RRoundBracket));
            p.bump();
        }
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(TokenKind::Plus) => Op::Add,
            Some(TokenKind::Minus) => Op::Sub,
            Some(TokenKind::Star) => Op::Mul,
            Some(TokenKind::Slash) => Op::Div,
            _ => return,
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < min_binding_power {
            return;
        }

        p.bump();

        p.start_node_at(checkpoint, TokenKind::InfixExpression);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

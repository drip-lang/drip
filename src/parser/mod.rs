pub mod expr;
pub mod operators;

use crate::lexer::{Lexer, TokenKind};
use crate::syntax::{Drip, SyntaxNode};
use color_eyre::Result;
use expr::expr;
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

pub struct Parse {
    node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Result<Parse> {
        self.start_node(TokenKind::Root);

        expr(&mut self);

        self.builder.finish_node();

        Ok(Parse {
            node: self.builder.finish(),
        })
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder.token(Drip::kind_to_raw(kind), text.into());
    }

    fn start_node(&mut self, kind: TokenKind) {
        self.builder.start_node(Drip::kind_to_raw(kind));
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: TokenKind) {
        self.builder
            .start_node_at(checkpoint, Drip::kind_to_raw(kind));
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = Parser::new(input).parse().unwrap();
        expected_tree.assert_eq(&parse.debug_tree())
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
            Root@0..3
              Integer@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "TestIdent1",
            expect![[r#"
            Root@0..10
              Ident@0..10 "TestIdent1""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
            Root@0..3
              InfixExpression@0..3
                Integer@0..1 "1"
                Plus@1..2 "+"
                Integer@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
            Root@0..7
              InfixExpression@0..7
                InfixExpression@0..5
                  InfixExpression@0..3
                    Integer@0..1 "1"
                    Plus@1..2 "+"
                    Integer@2..3 "2"
                  Plus@3..4 "+"
                  Integer@4..5 "3"
                Plus@5..6 "+"
                Integer@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
            Root@0..7
              InfixExpression@0..7
                InfixExpression@0..5
                  Integer@0..1 "1"
                  Plus@1..2 "+"
                  InfixExpression@2..5
                    Integer@2..3 "2"
                    Star@3..4 "*"
                    Integer@4..5 "3"
                Minus@5..6 "-"
                Integer@6..7 "4""#]],
        );
    }
}

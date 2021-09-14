#![allow(unused)]

pub mod error;
pub mod event;
pub mod grammar;
pub mod marker;
pub mod parser;
pub mod sink;
pub mod source;

use crate::error::ParseError;
use crate::parser::Parser;
use crate::sink::Sink;
use crate::source::Source;
use drip_lexer::Lexer;
use drip_syntax::SyntaxNode;
use rowan::GreenNode;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);
    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();

        let tree = format!("{:#?}", self.syntax());

        s.push_str(&tree[0..tree.len() - 1]);

        for error in &self.errors {
            s.push_str(&format!("\n{}", error))
        }

        s
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
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
  Literal@0..3
    Number@0..3 "123""#]],
        );
    }

    //     #[test]
    //     fn parse_variable_ref() {
    //         check(
    //             "TestIdent1",
    //             expect![[r#"
    // Root@0..10
    //   Literal@0..10
    //     Ident@0..10 "TestIdent1""#]],
    //         );
    //     }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  InfixExpr@0..3
    Literal@0..1
      Number@0..1 "1"
    Plus@1..2 "+"
    Literal@2..3
      Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      InfixExpr@0..3
        Literal@0..1
          Number@0..1 "1"
        Plus@1..2 "+"
        Literal@2..3
          Number@2..3 "2"
      Plus@3..4 "+"
      Literal@4..5
        Number@4..5 "3"
    Plus@5..6 "+"
    Literal@6..7
      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    InfixExpr@0..5
      Literal@0..1
        Number@0..1 "1"
      Plus@1..2 "+"
      InfixExpr@2..5
        Literal@2..3
          Number@2..3 "2"
        Star@3..4 "*"
        Literal@4..5
          Number@4..5 "3"
    Minus@5..6 "-"
    Literal@6..7
      Number@6..7 "4""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
Root@0..6
  InfixExpr@0..6
    PrefixExpr@0..3
      Minus@0..1 "-"
      Literal@1..3
        Number@1..3 "20"
    Plus@3..4 "+"
    Literal@4..6
      Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
Root@0..14
  RoundBracketExpr@0..14
    LRoundBracket@0..1 "("
    RoundBracketExpr@1..13
      LRoundBracket@1..2 "("
      RoundBracketExpr@2..12
        LRoundBracket@2..3 "("
        RoundBracketExpr@3..11
          LRoundBracket@3..4 "("
          RoundBracketExpr@4..10
            LRoundBracket@4..5 "("
            RoundBracketExpr@5..9
              LRoundBracket@5..6 "("
              Literal@6..8
                Number@6..8 "10"
              RRoundBracket@8..9 ")"
            RRoundBracket@9..10 ")"
          RRoundBracket@10..11 ")"
        RRoundBracket@11..12 ")"
      RRoundBracket@12..13 ")"
    RRoundBracket@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  InfixExpr@0..7
    Literal@0..1
      Number@0..1 "5"
    Star@1..2 "*"
    RoundBracketExpr@2..7
      LRoundBracket@2..3 "("
      InfixExpr@3..6
        Literal@3..4
          Number@3..4 "2"
        Plus@4..5 "+"
        Literal@5..6
          Number@5..6 "1"
      RRoundBracket@6..7 ")""#]],
        );
    }
}

use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LetKw) {
        Some(variable_def(p))
    } else {
        expr::expr(p)
    }
}

fn variable_def(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LetKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);
    p.expect(TokenKind::Equals);

    expr::expr(p);

    m.complete(p, SyntaxKind::VariableDef)
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_variable_definition() {
        check(
            "let foo = bar",
            expect![[r#"
Root@0..13
  VariableDef@0..13
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    Literal@10..13
      Ident@10..13 "bar""#]],
        );
    }

    #[test]
    fn recover_on_let_token() {
        check(
            "let a =\nlet b = a",
            expect![[r#"
Root@0..17
  VariableDef@0..8
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 "\n"
  VariableDef@8..17
    LetKw@8..11 "let"
    Whitespace@11..12 " "
    Ident@12..13 "b"
    Whitespace@13..14 " "
    Equals@14..15 "="
    Whitespace@15..16 " "
    Literal@16..17
      Ident@16..17 "a"
error at 8..11: expected default integer, identifier, '-' or '(', but found 'let'"#]],
        );
    }

    #[test]
    fn drip_check() {
        check(
            "let a = 10 + 3 * (4 - 3 * 3)",
            expect![[r#"
Root@0..28
  VariableDef@0..28
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    InfixExpr@8..28
      Literal@8..11
        Integer@8..10 "10"
        Whitespace@10..11 " "
      Plus@11..12 "+"
      Whitespace@12..13 " "
      InfixExpr@13..28
        Literal@13..15
          Integer@13..14 "3"
          Whitespace@14..15 " "
        Star@15..16 "*"
        Whitespace@16..17 " "
        RoundBracketExpr@17..28
          LRoundBracket@17..18 "("
          InfixExpr@18..27
            Literal@18..20
              Integer@18..19 "4"
              Whitespace@19..20 " "
            Minus@20..21 "-"
            Whitespace@21..22 " "
            InfixExpr@22..27
              Literal@22..24
                Integer@22..23 "3"
                Whitespace@23..24 " "
              Star@24..25 "*"
              Whitespace@25..26 " "
              Literal@26..27
                Integer@26..27 "3"
          LRoundBracket@27..28 ")""#]],
        );
    }
}

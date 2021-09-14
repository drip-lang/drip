use crate::marker::CompletedMarker;
use crate::parser::Parser;
use drip_lexer::TokenKind;
use drip_syntax::SyntaxKind;

mod decl;
mod expr;
mod func;
mod structs;
mod types;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();

    while !p.at_end() {
        decl::decl(p);
    }

    marker.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    #[cfg(test)]
    fn check(input: &str, expected_tree: expect_test::Expect) {
        let parse = crate::parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
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

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            "   9876",
            expect![[r#"
Root@0..7
  Whitespace@0..3 "   "
  Literal@3..7
    Number@3..7 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "999   ",
            expect![[r#"
Root@0..6
  Literal@0..6
    Number@0..3 "999"
    Whitespace@3..6 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            " 123     ",
            expect![[r#"
Root@0..9
  Whitespace@0..1 " "
  Literal@1..9
    Number@1..4 "123"
    Whitespace@4..9 "     ""#]],
        );
    }

    #[test]
    fn parse_simple_infix_expression() {
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
    fn parse_variable_ref() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  VariableRef@0..7
    Ident@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_left_associative_infix_expression() {
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
    fn parse_infix_expression_with_mixed_binding_power() {
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
    fn parse_infix_expression_with_whitespace() {
        check(
            " 1 +   2* 3 ",
            expect![[r#"
Root@0..12
  Whitespace@0..1 " "
  InfixExpr@1..12
    Literal@1..3
      Number@1..2 "1"
      Whitespace@2..3 " "
    Plus@3..4 "+"
    Whitespace@4..7 "   "
    InfixExpr@7..12
      Literal@7..8
        Number@7..8 "2"
      Star@8..9 "*"
      Whitespace@9..10 " "
      Literal@10..12
        Number@10..11 "3"
        Whitespace@11..12 " ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 // Add one
  + 10 // Add ten",
            expect![[r##"
Root@0..37
  Whitespace@0..1 "\n"
  InfixExpr@1..37
    InfixExpr@1..22
      Literal@1..5
        Number@1..2 "1"
        Whitespace@2..5 "\n  "
      Plus@5..6 "+"
      Whitespace@6..7 " "
      Literal@7..22
        Number@7..8 "1"
        Whitespace@8..9 " "
        Comment@9..19 "// Add one"
        Whitespace@19..22 "\n  "
    Plus@22..23 "+"
    Whitespace@23..24 " "
    Literal@24..37
      Number@24..26 "10"
      Whitespace@26..27 " "
      Comment@27..37 "// Add ten""##]],
        );
    }

    #[test]
    fn do_not_parse_operator_if_getting_rhs_failed() {
        check(
            "(1+",
            expect![[r#"
Root@0..3
  RoundBracketExpr@0..3
    LRoundBracket@0..1 "("
    InfixExpr@1..3
      Literal@1..2
        Number@1..2 "1"
      Plus@2..3 "+"
error at 2..3: expected number, identifier, '-' or '('
error at 2..3: expected ')'"#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "-10",
            expect![[r#"
Root@0..3
  PrefixExpr@0..3
    Minus@0..1 "-"
    Literal@1..3
      Number@1..3 "10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_binary_operators() {
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

    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "(foo",
            expect![[r#"
Root@0..4
  RoundBracketExpr@0..4
    LRoundBracket@0..1 "("
    VariableRef@1..4
      Ident@1..4 "foo"
error at 1..4: expected '+', '-', '*', '/' or ')'"#]],
        );
    }

    #[test]
    fn define_constant() {
        check(
            "a :: 10 + 3 * (4 - 3 * 3)",
            expect![[r#"
Root@0..25
  ConstDef@0..25
    Ident@0..1 "a"
    Whitespace@1..2 " "
    ConstKw@2..4 "::"
    Whitespace@4..5 " "
    InfixExpr@5..25
      Literal@5..8
        Number@5..7 "10"
        Whitespace@7..8 " "
      Plus@8..9 "+"
      Whitespace@9..10 " "
      InfixExpr@10..25
        Literal@10..12
          Number@10..11 "3"
          Whitespace@11..12 " "
        Star@12..13 "*"
        Whitespace@13..14 " "
        RoundBracketExpr@14..25
          LRoundBracket@14..15 "("
          InfixExpr@15..24
            Literal@15..17
              Number@15..16 "4"
              Whitespace@16..17 " "
            Minus@17..18 "-"
            Whitespace@18..19 " "
            InfixExpr@19..24
              Literal@19..21
                Number@19..20 "3"
                Whitespace@20..21 " "
              Star@21..22 "*"
              Whitespace@22..23 " "
              Literal@23..24
                Number@23..24 "3"
          RRoundBracket@24..25 ")""#]],
        );
    }

    #[test]
    fn define_var() {
        check(
            "y := 5 * -(8 + 2)",
            expect![[r#"
Root@0..17
  VariableDef@0..17
    Ident@0..1 "y"
    Whitespace@1..2 " "
    VariableKw@2..4 ":="
    Whitespace@4..5 " "
    InfixExpr@5..17
      Literal@5..7
        Number@5..6 "5"
        Whitespace@6..7 " "
      Star@7..8 "*"
      Whitespace@8..9 " "
      PrefixExpr@9..17
        Minus@9..10 "-"
        RoundBracketExpr@10..17
          LRoundBracket@10..11 "("
          InfixExpr@11..16
            Literal@11..13
              Number@11..12 "8"
              Whitespace@12..13 " "
            Plus@13..14 "+"
            Whitespace@14..15 " "
            Literal@15..16
              Number@15..16 "2"
          RRoundBracket@16..17 ")""#]],
        )
    }

    #[test]
    fn define_assignment() {
        check(
            "a = b + 3 * 4",
            expect![[r#"
Root@0..13
  AssignDef@0..13
    VariableRef@0..2
      Ident@0..1 "a"
      Whitespace@1..2 " "
    Equals@2..3 "="
    Whitespace@3..4 " "
    InfixExpr@4..13
      VariableRef@4..6
        Ident@4..5 "b"
        Whitespace@5..6 " "
      Plus@6..7 "+"
      Whitespace@7..8 " "
      InfixExpr@8..13
        Literal@8..10
          Number@8..9 "3"
          Whitespace@9..10 " "
        Star@10..11 "*"
        Whitespace@11..12 " "
        Literal@12..13
          Number@12..13 "4""#]],
        )
    }

    #[test]
    fn def_constant() {
        check(
            "a :: 10",
            expect![[r#"
Root@0..7
  ConstDef@0..7
    Ident@0..1 "a"
    Whitespace@1..2 " "
    ConstKw@2..4 "::"
    Whitespace@4..5 " "
    Literal@5..7
      Number@5..7 "10""#]],
        )
    }

    #[test]
    fn define_complete_empty_function() {
        check(
            "test_fn :: () { }",
            expect![[r#"
Root@0..17
  FnDef@0..17
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    LRoundBracket@11..12 "("
    RRoundBracket@12..13 ")"
    Whitespace@13..14 " "
    LCurlyBracket@14..15 "{"
    Whitespace@15..16 " "
    RCurlyBracket@16..17 "}""#]],
        )
    }

    #[test]
    fn define_empty_parameter_function() {
        check(
            "test_fn :: () -> i32 { }",
            expect![[r#"
Root@0..24
  FnDef@0..24
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    LRoundBracket@11..12 "("
    RRoundBracket@12..13 ")"
    Whitespace@13..14 " "
    Arrow@14..16 "->"
    Whitespace@16..17 " "
    FnReturnDef@17..21
      FnReturnTypeDef@17..21
        Type@17..21
          Ident@17..20 "i32"
          Whitespace@20..21 " "
    LCurlyBracket@21..22 "{"
    Whitespace@22..23 " "
    RCurlyBracket@23..24 "}""#]],
        )
    }

    #[test]
    fn define_parameter_empty_return_function() {
        check(
            "test_fn :: (x: i32, y: string) { }",
            expect![[r#"
Root@0..34
  FnDef@0..34
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    LRoundBracket@11..12 "("
    FnParamListDef@12..29
      FnParamDef@12..20
        Ident@12..13 "x"
        Colon@13..14 ":"
        Whitespace@14..15 " "
        Type@15..18
          Ident@15..18 "i32"
        Comma@18..19 ","
        Whitespace@19..20 " "
      FnParamDef@20..29
        Ident@20..21 "y"
        Colon@21..22 ":"
        Whitespace@22..23 " "
        Type@23..29
          Ident@23..29 "string"
    RRoundBracket@29..30 ")"
    Whitespace@30..31 " "
    LCurlyBracket@31..32 "{"
    Whitespace@32..33 " "
    RCurlyBracket@33..34 "}""#]],
        )
    }

    #[test]
    fn define_empty_parameter_multiple_return_function() {
        check(
            "test_fn :: () -> (i32, string) { }",
            expect![[r#"
Root@0..34
  FnDef@0..34
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    LRoundBracket@11..12 "("
    RRoundBracket@12..13 ")"
    Whitespace@13..14 " "
    Arrow@14..16 "->"
    Whitespace@16..17 " "
    FnReturnDef@17..31
      LRoundBracket@17..18 "("
      FnReturnTypeListDef@18..29
        FnReturnTypeDef@18..21
          Type@18..21
            Ident@18..21 "i32"
        Comma@21..22 ","
        Whitespace@22..23 " "
        FnReturnTypeDef@23..29
          Type@23..29
            Ident@23..29 "string"
      RRoundBracket@29..30 ")"
      Whitespace@30..31 " "
    LCurlyBracket@31..32 "{"
    Whitespace@32..33 " "
    RCurlyBracket@33..34 "}""#]],
        )
    }

    #[test]
    fn define_parameter_return_body_function() {
        check(
            r#"
                test_fn :: (a: i32, b: i32) -> i32 {
                    a + b
                }"#,
            expect![[r#"
Root@0..97
  Whitespace@0..17 "\n                "
  FnDef@17..97
    Ident@17..24 "test_fn"
    Whitespace@24..25 " "
    ConstKw@25..27 "::"
    Whitespace@27..28 " "
    LRoundBracket@28..29 "("
    FnParamListDef@29..43
      FnParamDef@29..37
        Ident@29..30 "a"
        Colon@30..31 ":"
        Whitespace@31..32 " "
        Type@32..35
          Ident@32..35 "i32"
        Comma@35..36 ","
        Whitespace@36..37 " "
      FnParamDef@37..43
        Ident@37..38 "b"
        Colon@38..39 ":"
        Whitespace@39..40 " "
        Type@40..43
          Ident@40..43 "i32"
    RRoundBracket@43..44 ")"
    Whitespace@44..45 " "
    Arrow@45..47 "->"
    Whitespace@47..48 " "
    FnReturnDef@48..52
      FnReturnTypeDef@48..52
        Type@48..52
          Ident@48..51 "i32"
          Whitespace@51..52 " "
    LCurlyBracket@52..53 "{"
    Whitespace@53..74 "\n                    "
    FnBodyDef@74..96
      InfixExpr@74..96
        VariableRef@74..76
          Ident@74..75 "a"
          Whitespace@75..76 " "
        Plus@76..77 "+"
        Whitespace@77..78 " "
        VariableRef@78..96
          Ident@78..79 "b"
          Whitespace@79..96 "\n                "
    RCurlyBracket@96..97 "}""#]],
        )
    }

    #[test]
    fn define_empty_struct() {
        check(
            "TestStruct :: struct { }",
            expect![[r#"
Root@0..24
  StructDef@0..24
    Ident@0..10 "TestStruct"
    Whitespace@10..11 " "
    ConstKw@11..13 "::"
    Whitespace@13..14 " "
    StructKw@14..20 "struct"
    Whitespace@20..21 " "
    StructFieldListDef@21..24
      LCurlyBracket@21..22 "{"
      Whitespace@22..23 " "
      RCurlyBracket@23..24 "}""#]],
        )
    }

    #[test]
    fn define_struct_with_fields() {
        check(
            "NamedVector2 :: struct { name: string x: f32 y: f32 }",
            expect![[r#"
Root@0..53
  StructDef@0..53
    Ident@0..12 "NamedVector2"
    Whitespace@12..13 " "
    ConstKw@13..15 "::"
    Whitespace@15..16 " "
    StructKw@16..22 "struct"
    Whitespace@22..23 " "
    StructFieldListDef@23..53
      LCurlyBracket@23..24 "{"
      Whitespace@24..25 " "
      StructFieldDef@25..38
        Ident@25..29 "name"
        Colon@29..30 ":"
        Whitespace@30..31 " "
        Type@31..38
          Ident@31..37 "string"
          Whitespace@37..38 " "
      StructFieldDef@38..45
        Ident@38..39 "x"
        Colon@39..40 ":"
        Whitespace@40..41 " "
        Type@41..45
          Ident@41..44 "f32"
          Whitespace@44..45 " "
      StructFieldDef@45..52
        Ident@45..46 "y"
        Colon@46..47 ":"
        Whitespace@47..48 " "
        Type@48..52
          Ident@48..51 "f32"
          Whitespace@51..52 " "
      RCurlyBracket@52..53 "}""#]],
        );
    }

    #[test]
    fn define_struct_with_fields_and_commas() {
        check(
            "NamedVector2 :: struct { name: string, x: f32, y: f32, }",
            expect![[r#"
Root@0..56
  StructDef@0..56
    Ident@0..12 "NamedVector2"
    Whitespace@12..13 " "
    ConstKw@13..15 "::"
    Whitespace@15..16 " "
    StructKw@16..22 "struct"
    Whitespace@22..23 " "
    StructFieldListDef@23..56
      LCurlyBracket@23..24 "{"
      Whitespace@24..25 " "
      StructFieldDef@25..39
        Ident@25..29 "name"
        Colon@29..30 ":"
        Whitespace@30..31 " "
        Type@31..37
          Ident@31..37 "string"
        Comma@37..38 ","
        Whitespace@38..39 " "
      StructFieldDef@39..47
        Ident@39..40 "x"
        Colon@40..41 ":"
        Whitespace@41..42 " "
        Type@42..45
          Ident@42..45 "f32"
        Comma@45..46 ","
        Whitespace@46..47 " "
      StructFieldDef@47..55
        Ident@47..48 "y"
        Colon@48..49 ":"
        Whitespace@49..50 " "
        Type@50..53
          Ident@50..53 "f32"
        Comma@53..54 ","
        Whitespace@54..55 " "
      RCurlyBracket@55..56 "}""#]],
        );
    }

    #[test]
    fn define_empty_trait() {
        check(
            "TestTrait :: trait { }",
            expect![[r#"
Root@0..22
  TraitDef@0..22
    Ident@0..9 "TestTrait"
    Whitespace@9..10 " "
    ConstKw@10..12 "::"
    Whitespace@12..13 " "
    TraitKw@13..18 "trait"
    Whitespace@18..19 " "
    TraitListsDef@19..22
      LCurlyBracket@19..20 "{"
      Whitespace@20..21 " "
      RCurlyBracket@21..22 "}""#]],
        );
    }
}

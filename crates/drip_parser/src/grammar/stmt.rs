use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::Ident) {
        Some(identifier(p))
    } else {
        expr::expr(p)
    }
}

fn identifier(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Ident));
    let marker = p.start();
    p.bump();

    if p.at(TokenKind::ConstKw) {
        const_def(p);
        marker.complete(p, SyntaxKind::ConstDef)
    } else if p.at(TokenKind::VariableKw) {
        variable_def(p);
        marker.complete(p, SyntaxKind::VariableDef)
    } else if p.at(TokenKind::Equals) {
        assign_def(p);
        marker.complete(p, SyntaxKind::AssignDef)
    } else {
        expr::expr(p).unwrap()
    }
}

fn const_def(p: &mut Parser) {
    assert!(p.at(TokenKind::ConstKw));
    p.bump();

    if p.at(TokenKind::StructKw) {
        struct_def(p);
    } else if p.at(TokenKind::TraitKw) {
        trait_def(p);
    } else if p.at(TokenKind::FnKw) {
        function_def(p);
    } else {
        expr::expr(p);
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

// TODO: Get rid of FnKw. Maybe match until it's sure if it's an expr or fn and return that up
fn function_def(p: &mut Parser) {
    assert!(p.at(TokenKind::FnKw));
    let m = p.start();
    p.bump();

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
    m.complete(p, SyntaxKind::FnDef);
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
    stmt(p);
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

fn struct_def(p: &mut Parser) {
    assert!(p.at(TokenKind::StructKw));
    let m = p.start();
    p.bump();
    struct_field_list_def(p);
    m.complete(p, SyntaxKind::StructDef);
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

fn trait_def(p: &mut Parser) {
    assert!(p.at(TokenKind::TraitKw));
    let m = p.start();
    p.bump();
    trait_lists_def(p);
    m.complete(p, SyntaxKind::TraitDef);
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
    stmt(p);
    m.complete(p, SyntaxKind::TraitFnListDef);
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
    Ident@0..1 "a"
    Whitespace@1..2 " "
    Equals@2..3 "="
    Whitespace@3..4 " "
    InfixExpr@4..13
      Literal@4..6
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
    fn define_complete_empty_function() {
        check(
            "test_fn :: fn() { }",
            expect![[r#"
Root@0..19
  ConstDef@0..19
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    FnDef@11..19
      FnKw@11..13 "fn"
      LRoundBracket@13..14 "("
      RRoundBracket@14..15 ")"
      Whitespace@15..16 " "
      LCurlyBracket@16..17 "{"
      Whitespace@17..18 " "
      RCurlyBracket@18..19 "}""#]],
        )
    }

    #[test]
    fn define_empty_parameter_function() {
        check(
            "test_fn :: fn() -> i32 { }",
            expect![[r#"
Root@0..26
  ConstDef@0..26
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    FnDef@11..26
      FnKw@11..13 "fn"
      LRoundBracket@13..14 "("
      RRoundBracket@14..15 ")"
      Whitespace@15..16 " "
      Arrow@16..18 "->"
      Whitespace@18..19 " "
      FnReturnDef@19..23
        FnReturnTypeDef@19..23
          Type@19..23
            Ident@19..22 "i32"
            Whitespace@22..23 " "
      LCurlyBracket@23..24 "{"
      Whitespace@24..25 " "
      RCurlyBracket@25..26 "}""#]],
        )
    }

    #[test]
    fn define_parameter_empty_return_function() {
        check(
            "test_fn :: fn(x: i32, y: string) { }",
            expect![[r#"
Root@0..36
  ConstDef@0..36
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    FnDef@11..36
      FnKw@11..13 "fn"
      LRoundBracket@13..14 "("
      FnParamListDef@14..31
        FnParamDef@14..22
          Ident@14..15 "x"
          Colon@15..16 ":"
          Whitespace@16..17 " "
          Type@17..20
            Ident@17..20 "i32"
          Comma@20..21 ","
          Whitespace@21..22 " "
        FnParamDef@22..31
          Ident@22..23 "y"
          Colon@23..24 ":"
          Whitespace@24..25 " "
          Type@25..31
            Ident@25..31 "string"
      RRoundBracket@31..32 ")"
      Whitespace@32..33 " "
      LCurlyBracket@33..34 "{"
      Whitespace@34..35 " "
      RCurlyBracket@35..36 "}""#]],
        )
    }

    #[test]
    fn define_empty_parameter_multiple_return_function() {
        check(
            "test_fn :: fn() -> (i32, string) { }",
            expect![[r#"
Root@0..36
  ConstDef@0..36
    Ident@0..7 "test_fn"
    Whitespace@7..8 " "
    ConstKw@8..10 "::"
    Whitespace@10..11 " "
    FnDef@11..36
      FnKw@11..13 "fn"
      LRoundBracket@13..14 "("
      RRoundBracket@14..15 ")"
      Whitespace@15..16 " "
      Arrow@16..18 "->"
      Whitespace@18..19 " "
      FnReturnDef@19..33
        LRoundBracket@19..20 "("
        FnReturnTypeListDef@20..31
          FnReturnTypeDef@20..23
            Type@20..23
              Ident@20..23 "i32"
          Comma@23..24 ","
          Whitespace@24..25 " "
          FnReturnTypeDef@25..31
            Type@25..31
              Ident@25..31 "string"
        RRoundBracket@31..32 ")"
        Whitespace@32..33 " "
      LCurlyBracket@33..34 "{"
      Whitespace@34..35 " "
      RCurlyBracket@35..36 "}""#]],
        )
    }

    //     #[test]
    //     fn define_parameter_return_body_function() {
    //         check(
    //             r#"
    //             test_fn :: fn(a: i32, b: i32) -> i32 {
    //                 a + b
    //             }"#,
    //             expect![[r#"
    // "#]])
    //     }

    #[test]
    fn define_empty_struct() {
        check(
            "TestStruct :: struct { }",
            expect![[r#"
Root@0..24
  ConstDef@0..24
    Ident@0..10 "TestStruct"
    Whitespace@10..11 " "
    ConstKw@11..13 "::"
    Whitespace@13..14 " "
    StructDef@14..24
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
  ConstDef@0..53
    Ident@0..12 "NamedVector2"
    Whitespace@12..13 " "
    ConstKw@13..15 "::"
    Whitespace@15..16 " "
    StructDef@16..53
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
  ConstDef@0..56
    Ident@0..12 "NamedVector2"
    Whitespace@12..13 " "
    ConstKw@13..15 "::"
    Whitespace@15..16 " "
    StructDef@16..56
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
  ConstDef@0..22
    Ident@0..9 "TestTrait"
    Whitespace@9..10 " "
    ConstKw@10..12 "::"
    Whitespace@12..13 " "
    TraitDef@13..22
      TraitKw@13..18 "trait"
      Whitespace@18..19 " "
      TraitListsDef@19..22
        LCurlyBracket@19..20 "{"
        Whitespace@20..21 " "
        RCurlyBracket@21..22 "}""#]],
        );
    }

    // #[test]
    // fn define_trait_with_functions() {
    //     check(
    //         r#"
    //         Sound :: trait {
    //             make_sound :: ( )
    //         }
    //         "#,
    //         expect![[r#""#]]);
    // }
}

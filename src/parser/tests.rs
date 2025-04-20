use crate::parser::ast::{Expression, Literal, Statement};
use crate::token::Token;
use crate::{lexer::Lexer, parser::Parser};

#[test]
fn test_variable_declaration() {
    let input = "actualize synergyScore align 100";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());
    let program = result.unwrap();

    assert_eq!(program.body.len(), 1);
    match &program.body[0] {
        Statement::VariableDeclaration { identifier, value } => {
            assert_eq!(identifier.value, "synergyScore");
            assert!(matches!(value, Expression::Literal(Literal::Number(100))));
        }
        _ => panic!("Expected VariableDeclaration"),
    }
}

#[test]
fn test_return_statements() {
    let input = "return 100";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());
    let program = result.unwrap();

    assert_eq!(program.body.len(), 1);
    match &program.body[0] {
        Statement::ReturnStatement(value) => {
            assert!(matches!(value, Expression::Literal(Literal::Number(100))));
        }
        _ => panic!("Expected VariableDeclaration"),
    }
}

#[test]
fn test_unary_expression() {
    let input = "-100";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());
    let program = result.unwrap();

    assert_eq!(program.body.len(), 1);
    match &program.body[0] {
        Statement::ExpressionStatement(value) => match value {
            Expression::Unary { operator, right } => {
                assert_eq!(operator, &Token::MINUS);
                match &**right {
                    Expression::Literal(literal) => {
                        assert_eq!(literal, &Literal::Number(100))
                    }
                    _ => panic!("Expected VariableDeclaration"),
                }
            }
            _ => panic!("Expected VariableDeclaration"),
        },
        _ => panic!("Expected VariableDeclaration"),
    }
}

#[test]
fn test_unary_expression_negate() {
    let input = "!true";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());
    let program = result.unwrap();

    print!("{}", program);

    assert_eq!(program.body.len(), 1);
    match &program.body[0] {
        Statement::ExpressionStatement(value) => match value {
            Expression::Unary { operator, right } => {
                assert_eq!(operator, &Token::NEGATE);
                match &**right {
                    Expression::Literal(literal) => {
                        assert_eq!(literal, &Literal::Boolean(true))
                    }
                    _ => panic!("Expected Expression Literal"),
                }
            }
            _ => panic!("Expected Expression Unary"),
        },
        _ => panic!("Expected Expression Statement"),
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let inputs = [
        ("5 + 5", 5, Token::PLUS, 5),
        ("5 value-add 5", 5, Token::PLUS, 5),
        ("5 - 5", 5, Token::MINUS, 5),
        ("5 * 5", 5, Token::MULTIPLY, 5),
        ("5 / 5", 5, Token::DIVIDE, 5),
        ("5 % 5", 5, Token::MODULO, 5),
        ("5 > 5", 5, Token::GT, 5),
        ("5 < 5", 5, Token::LT, 5),
        ("5 == 5", 5, Token::EQ, 5),
        ("5 != 5", 5, Token::NEQ, 5),
    ];

    for (input, l, t, r) in inputs {
        let lexer = Lexer::new(&input);
        let mut p = Parser::new(lexer);

        let result = p.parse();

        assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());

        let program = result.unwrap();

        assert_eq!(program.body.len(), 1);

        match &program.body[0] {
            Statement::ExpressionStatement(value) => match value {
                Expression::Binary {
                    left,
                    operator,
                    right,
                } => {
                    assert_eq!(left.to_string(), l.to_string());
                    assert_eq!(operator, &t);
                    assert_eq!(right.to_string(), r.to_string());
                }

                _ => panic!("Expected Binary"),
            },
            _ => panic!("Expected Expression Statement"),
        }
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = [
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("-1 + 2", "((-1) + 2)"),
        ("1 + -2", "(1 + (-2))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 > 4 != 3 < 4", "((5 > 4) != (3 < 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("5 value-add 5", "(5 + 5)"),
        ("5 value-add 5 cut 7", "((5 + 5) - 7)"),
        ("5 value-add 5 streamline 7", "((5 + 5) - 7)"),
        ("5 value-add (5 streamline 7)", "(5 + (5 - 7))"),
        ("5 is 5", "(5 == 5)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(&input);
        let mut p = Parser::new(l);

        let result = p.parse();

        assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());
        let program = result.unwrap();

        assert_eq!(program.to_string(), expected);
    }
}

#[test]
fn test_conditional_with_else() {
    let input = r#"
evaluate synergyScore greater than 50
	1 + 2
pivot
	3 - 4
end
"#;
    //     let input = r#"
    // evaluate synergyScore greater than 50
    // 	touch base with "On track"
    // pivot
    // 	touch base with "Needs optimization"
    // end
    // "#;
    // let input = r#"
    //     evaluate synergyScore greater than 50
    //     end
    // "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_ok(), "Parser returned errors: {:?}", result.err());
    let program = result.unwrap();

    assert_eq!(program.body.len(), 1);
    match &program.body[0] {
        Statement::ExpressionStatement(value) => match value {
            Expression::Conditional {
                condition,
                then_branch,
                elif_branch,
                else_branch,
            } => {
                match &**condition {
                    Expression::Binary { operator, .. } => assert_eq!(operator, &Token::GT),
                    _ => panic!("Expected binary expression for condition"),
                }
                assert_eq!(then_branch.statements.len(), 1);
                assert!(elif_branch.is_none());
                assert_eq!(else_branch.as_ref().unwrap().statements.len(), 1);
            }
            _ => panic!("Expected Conditional"),
        },
        _ => panic!("Expected Conditional"),
    }
}

#[test]
fn test_error_handling_invalid_syntax() {
    let input = "actualize align 123"; // Missing identifier
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(errors.msg.contains("Expected Identifier"))
}

#[test]
fn test_conditional_without_else() {
    let input = r#"
evaluate score above 10
	let x = 5
end
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    assert_eq!(program.body.len(), 1);
    // match &program.body[0] {
    //     Expression::Conditional {
    //         condition,
    //         then_branch,
    //         else_branch,
    //     } => {
    //         assert_eq!(then_branch.statements.len(), 1);
    //         assert!(else_branch.is_none());
    //         match condition {
    //             Expression::Binary { operator, .. } => assert_eq!(operator, &Operator::GT),
    //             _ => panic!("Expected binary condition"),
    //         }
    //     }
    //     _ => panic!("Expected Conditional"),
    // }
}

#[test]
fn test_nested_conditionals() {
    let input = r#"
evaluate outerScore greater than 5
	evaluate innerScore greater than 3
		1 + 2
	pivot
		3 + 4
	end
pivot
	5 + 6
end
"#;

    // end
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();
    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    // print!("PROGRAM: {:#?}", program);

    assert_eq!(program.body.len(), 1);
    // match &program.body[0] {
    //     Statement::Conditional {
    //         then_branch,
    //         else_branch,
    //         ..
    //     } => {
    //         assert_eq!(then_branch.statements.len(), 1);
    //         match &then_branch.statements[0] {
    //             Statement::Conditional {
    //                 then_branch: inner_then,
    //                 else_branch: inner_else,
    //                 ..
    //             } => {
    //                 assert_eq!(inner_then.statements.len(), 1);
    //                 assert_eq!(inner_else.as_ref().unwrap().statements.len(), 1);
    //             }
    //             _ => panic!("Expected nested Conditional"),
    //         }
    //         assert_eq!(else_branch.as_ref().unwrap().statements.len(), 1);
    //     }
    //     _ => panic!("Expected outer Conditional"),
    // }
}

#[test]
fn test_conditional_with_no_body() {
    let input = r#"
evaluate x exceeds 0
end
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    assert_eq!(program.body.len(), 1);
    // match &program.body[0] {
    //     Expression::Conditional {
    //         then_branch,
    //         else_branch,
    //         ..
    //     } => {
    //         assert!(then_branch.statements.is_empty());
    //         assert!(else_branch.is_none());
    //     }
    //     _ => panic!("Expected Conditional"),
    // }
}

#[test]
fn test_function_literal_parsing() {
    let input = r#"
plan a b to
	let c = a + b
	deliver c
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    print!("FN {:#?}", program);

    assert_eq!(program.body.len(), 1);
}

#[test]
fn test_function_literal_with_no_parameters_parsing() {
    let input = r#"
plan to
	deliver 5
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    print!("FN {:#?}", program);

    assert_eq!(program.body.len(), 1);
}

#[test]
fn test_function_literal_with_no_parameters_inline_parsing() {
    let input = r#"
plan to deliver 5
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    print!("FN {:#?}", program);

    assert_eq!(program.body.len(), 1);
}

#[test]
fn test_call_expression() {
    let input = r#"
boost_moral leverage "pizza party"
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let result = parser.parse();

    assert!(
        result.is_ok(),
        "Unexpected error: {:?}",
        result.unwrap_err()
    );
    let program = result.unwrap();

    print!("FN {:#?}", program);

    assert_eq!(program.body.len(), 1);
}

// #[test]
// fn test_loop_with_circle_back_and_pivotaway() {
//     let input = r#"
//         optimize while counter less than 5
//           circle back
//           punt
//         end
//     "#;

//     let lexer = Lexer::new(input);
//     let mut parser = Parser::new(lexer);
//     let result = parser.parse();

//     assert!(
//         result.is_ok(),
//         "Parser returned errors: {:?}",
//         result.unwrap_err()
//     );
//     let program = result.unwrap();

//     assert_eq!(program.body.len(), 1);
//     match &program.body[0] {
//         Statement::Loop { condition, body } => {
//             match condition {
//                 Expression::Binary { operator, .. } => assert_eq!(operator, &Operator::LT),
//                 _ => panic!("Expected binary expression for loop condition"),
//             }
//             assert_eq!(body.statements.len(), 2);
//             assert!(matches!(body.statements[0], Statement::Continue));
//             assert!(matches!(body.statements[1], Statement::Break));
//         }
//         _ => panic!("Expected Loop statement"),
//     }
// }

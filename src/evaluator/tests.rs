use std::fmt::Error;

use crate::evaluator::eval;
use crate::object::Object;
use crate::parser::ast::{Expression, Literal, Statement};
use crate::{lexer::Lexer, parser::Parser};

fn setup_eval(input: &str) -> Result<Object, Error> {
    let l = Lexer::new(&input);
    let mut p = Parser::new(l);
    let program = p.parse();

    if program.is_ok() {
        let eval_result = eval(program.unwrap());

        if eval_result.is_ok() {
            Ok(eval_result.unwrap())
        } else {
            Err(Error {})
        }
    } else {
        Err(Error {})
    }
}

#[test]
fn test_integer_eval() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("0", 0),
        ("100", 100),
        ("42", 42),
        ("-1", -1),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 +-10", 50),
    ];

    for (input, expected) in tests {
        let eval_result = setup_eval(input);
        assert!(
            eval_result.is_ok(),
            "Evaluator returned errors: {:?}",
            eval_result.err()
        );

        let result = eval_result.unwrap();
        assert_eq!(result, Object::Integer(expected));
    }
}

#[test]
fn test_boolean_eval() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("actionable", true),
        ("headwinds", false),
    ];

    for (input, expected) in tests {
        let eval_result = setup_eval(input);
        assert!(
            eval_result.is_ok(),
            "Evaluator returned errors: {:?}",
            eval_result.err()
        );

        let result = eval_result.unwrap();
        assert_eq!(result, Object::Boolean(expected));
    }
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("not true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
        ("actionable", true),
        ("!actionable", false),
        ("not actionable", false),
    ];

    for (input, expected) in tests {
        let eval_result = setup_eval(input);
        assert!(
            eval_result.is_ok(),
            "Evaluator returned errors: {:?}",
            eval_result.err()
        );

        let result = eval_result.unwrap();
        assert_eq!(result, Object::Boolean(expected));
    }
}

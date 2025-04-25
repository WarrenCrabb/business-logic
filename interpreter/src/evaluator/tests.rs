use std::fmt::Error;

use crate::object::environment::Env;
use crate::object::{NULL, Object};
use crate::{lexer::Lexer, parser::Parser};

use super::Evaluator;

fn setup_eval(input: &str) -> Result<Object, Error> {
    let l = Lexer::new(&input);
    let mut p = Parser::new(l);
    let program = p.parse();

    if program.is_ok() {
        let env = Env::default();

        let mut evaluator = Evaluator::default();

        let eval_result = evaluator.eval(program.unwrap(), &env);

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
        // ("*10", 10),
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
            eval_result.unwrap_err()
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
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 > 1", false),
        ("1 < 1", false),
        ("1 >= 1", true),
        ("1 <= 1", true),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 != 2", true),
        ("1 == 2", false),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
        ("true && true", true),
        ("true && false", false),
        ("false && true", false),
        ("false && false", false),
        ("true || true", true),
        ("true || false", true),
        ("false || true", true),
        ("false || false", false),
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

#[test]
fn test_conditional_expressions() {
    let tests = vec![
        (
            r#"
        if true 
          10
        end
        "#,
            Object::Integer(10),
        ),
        (
            r#"
        if 1 
          10
        end
        "#,
            Object::Integer(10),
        ),
        (
            r#"
        if false 
          10
        end
        "#,
            NULL,
        ),
        (
            r#"
        evaluate 1 > 2
          10
        pivot 
          20
        end
        "#,
            Object::Integer(20),
        ),
        (
            r#"
        evaluate 1 < 2
          10
        pivot 
          20
        end
        "#,
            Object::Integer(10),
        ),
    ];

    for (input, expected) in tests {
        let eval_result = setup_eval(input);

        if eval_result.is_ok() {
            let result = eval_result.unwrap();
            assert_eq!(result, expected);
        } else {
            let result = eval_result.unwrap();
            assert_eq!(result, Object::Null);
        }
    }
}

#[test]
fn test_return_statement() {
    let tests = vec![
        ("return 10", Object::Integer(10)),
        ("return 10 9", Object::Integer(10)),
        ("return 2 * 5", Object::Integer(10)),
        ("9 return 2 * 5 9", Object::Integer(10)),
        (
            r#"
            if 10 > 1
              if 10 > 1 
                return 10 
              end
              return 1
            end
        "#,
            Object::Integer(10),
        ),
    ];

    for (input, expected) in tests {
        let eval_result = setup_eval(input);

        if eval_result.is_ok() {
            let result = eval_result.unwrap();
            assert_eq!(result, expected);
        } else {
            let result = eval_result.unwrap();
            assert_eq!(result, Object::Null);
        }
    }
}

#[test]
fn test_let_statement() {
    let tests = vec![
        ("let a = 5\na", 5),
        ("let a = 5 * 5\na", 25),
        ("let a = 5\nlet b = a\nb", 5),
        ("let a = 5\nlet b = a\nlet c = a + b + 5\nc", 15),
    ];

    for (input, expected) in tests {
        let eval_result = setup_eval(input);

        // print!("EVAL: {:?} ", eval_result);

        if eval_result.is_ok() {
            let result = eval_result.unwrap();
            assert_eq!(result, Object::Integer(expected));
        } else {
            let result = eval_result.unwrap();
            assert_eq!(result, Object::Null);
        }
    }
}

#[test]
fn test_function_object() {
    let tests = vec![(
        "let identity = plan x to\n\tdeliver x + x\nidentity leverage 3\n",
        6,
    )];

    let script = r#"
    actualize adder to plan a b align
      let c = a + b
      deliver c

    execute adder leverage 5 5   
    "#;

    let eval_result = setup_eval(script);

    if eval_result.is_err() {
        let e = eval_result.unwrap_err();
        print!("SCRIPT: {}", e);
    } else {
        let e = eval_result.unwrap();
        print!("SCRIPT: {}", e);
    }

    for (input, expected) in tests {
        let eval_result = setup_eval(input);

        // print!("EVAL: {:?} ", eval_result);

        if eval_result.is_ok() {
            let result = eval_result.unwrap();
            assert_eq!(result, Object::Integer(expected));
        } else {
            let result = eval_result.unwrap();
            assert_eq!(result, Object::Null);
        }
    }
}

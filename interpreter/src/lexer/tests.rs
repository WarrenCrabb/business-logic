use crate::{lexer::Lexer, token::Token};

#[test]
fn test_next_token() {
    let input = "let x = 5";

    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("x")),
        Token::BIND,
        Token::NUMBER(5),
        Token::EOF,
    ];
    let mut lexer = Lexer::new(input);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_operator_tokens() {
    let input = "initiate x = 5 + 5";

    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("x")),
        Token::BIND,
        Token::NUMBER(5),
        Token::PLUS,
        Token::NUMBER(5),
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_multiword_operator_tokens() {
    let input = "initiate x = 5 greater than 5";

    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("x")),
        Token::BIND,
        Token::NUMBER(5),
        Token::GT,
        Token::NUMBER(5),
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_more_tokens() {
    let source = r#"
strategize main with no deliverables
    actualize synergyScore align 100
	  print with "Quarterly optimization"

	while momentum below synergyScore
		print with "Driving momentum..."
		momentum align momentum value_add 10
	end
end

execute
"#;

    let tests = vec![
        // Token::BLANK,
        Token::FUNCTION,
        Token::IDENTIFIER(String::from("main")),
        Token::BIND,
        // Token::KEYWORD(String::from("with")),
        Token::IDENTIFIER(String::from("no")),
        Token::IDENTIFIER(String::from("deliverables")),
        Token::EOL,
        Token::INDENT,
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::BIND,
        Token::NUMBER(100),
        Token::EOL,
        // Token::STDOUT,
        Token::IDENTIFIER(String::from("print")),
        // Token::IDENTIFIER(String::from("touch")),
        // Token::IDENTIFIER(String::from("base")),
        Token::BIND,
        // Token::KEYWORD(String::from("with")),
        Token::STRING(String::from("Quarterly optimization")),
        Token::EOL,
        Token::BLANK,
        // Token::IDENTIFIER(String::from("optimize")),
        // Token::IDENTIFIER(String::from("while")),
        Token::DEDENT,
        Token::WHILE,
        Token::IDENTIFIER(String::from("momentum")),
        Token::LT,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::EOL,
        Token::INDENT,
        // Token::STDOUT,
        Token::IDENTIFIER(String::from("print")),
        Token::BIND,
        // Token::KEYWORD(String::from("with")),
        Token::STRING(String::from("Driving momentum...")),
        Token::EOL,
        Token::IDENTIFIER(String::from("momentum")),
        Token::BIND,
        Token::IDENTIFIER(String::from("momentum")),
        Token::PLUS,
        Token::NUMBER(10),
        Token::EOL,
        Token::DEDENT,
        Token::END,
        Token::EOL,
        Token::DEDENT,
        Token::END,
        Token::EOL,
        Token::BLANK,
        Token::RETURN,
        // Token::KEYWORD(String::from("execute")),
        // Token::EOL,
        Token::EOF,
    ];

    let mut lexer = Lexer::new(source);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_assignment_and_operators() {
    let input = "actualize synergyScore to 100 value_add 10";
    let mut lexer = Lexer::new(input);
    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::BIND,
        Token::NUMBER(100),
        Token::PLUS,
        Token::NUMBER(10),
        Token::EOF,
    ];

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_function_literal() {
    let input = "plan synergyScore to";
    let mut lexer = Lexer::new(input);
    let tests = vec![
        Token::FUNCTION,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::BIND,
        Token::EOF,
    ];

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_function_call() {
    let input = "boost_moral leverage \"free lunch\"";
    let mut lexer = Lexer::new(input);
    let tests = vec![
        Token::IDENTIFIER(String::from("boost_moral")),
        Token::CALL,
        Token::STRING(String::from("free lunch")),
        Token::EOF,
    ];

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_conditional_branches() {
    let input = r#"
evaluate x > 5
	false
re_evaluate x above 3
	false
pivot
	true 
"#;
    let mut lexer = Lexer::new(input);
    let tests = vec![
        // Token::BLANK,
        Token::IF,
        Token::IDENTIFIER(String::from("x")),
        Token::GT,
        Token::NUMBER(5),
        Token::EOL,
        Token::INDENT,
        Token::BOOLEAN(false),
        Token::EOL,
        Token::DEDENT,
        Token::ELIF,
        Token::IDENTIFIER(String::from("x")),
        Token::GT,
        Token::NUMBER(3),
        Token::EOL,
        Token::INDENT,
        Token::BOOLEAN(false),
        Token::EOL,
        Token::DEDENT,
        Token::ELSE,
        Token::EOL,
        Token::INDENT,
        Token::BOOLEAN(true),
        Token::EOF,
    ];

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

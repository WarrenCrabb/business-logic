use std::io::{self, Write};

use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::object::environment::Env;
use crate::parser::Parser;

pub fn run_script<W: Write>(script: &str, mut writer: W) -> io::Result<()> {
    let env = Env::default();
    let mut evaluator = Evaluator::default();
    let lexer = Lexer::new(script);
    let mut parser = Parser::new(lexer);
    let parser_result = parser.parse();

    if parser_result.is_err() {
        let e = parser_result.unwrap_err();
        writeln!(writer, "ERROR: {} ", e.msg)?;
        return Ok(());
    }

    let program = parser_result.unwrap();

    let evaluated = evaluator.eval(program, &env);

    if evaluated.is_err() {
        let e = evaluated.unwrap_err();
        writeln!(writer, "ERROR: {} ", e.msg)?;
        return Ok(());
    }

    let result = evaluated.unwrap();

    print!("{}", evaluator.output_buffer);
    evaluator.flush_output_buffer();

    writeln!(writer, "{}", result)?;
    Ok(())
}

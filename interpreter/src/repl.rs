use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::object::environment::Env;
use crate::parser::Parser;

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let env = Env::default();
    let mut evaluator = Evaluator::default();
    loop {
        // Write the prompt.
        write!(writer, "{}", PROMPT)?;
        writer.flush()?;

        // Read one line from input.
        let mut line = String::new();

        if reader.read_line(&mut line)? == 0 {
            // No more input (EOF reached).
            break;
        }

        // Create a new lexer with the input line.
        let lexer = Lexer::new(&line);
        let mut parser = Parser::new(lexer);
        let parser_result = parser.parse();

        if parser_result.is_err() {
            let e = parser_result.unwrap_err();
            writeln!(writer, "ERROR: {} ", e.msg)?;
            continue;
        }

        let program = parser_result.unwrap();

        let evaluated = evaluator.eval(program, &env);

        if evaluated.is_err() {
            let e = evaluated.unwrap_err();
            writeln!(writer, "ERROR: {} ", e.msg)?;
            continue;
        }

        let result = evaluated.unwrap();

        print!("{}", evaluator.output_buffer);
        evaluator.flush_output_buffer();

        writeln!(writer, "{}", result)?;
    }
    Ok(())
}

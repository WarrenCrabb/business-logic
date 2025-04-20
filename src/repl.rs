use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
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
        let result = parser.parse();

        if result.is_err() {
            let e = result.unwrap_err();
            writeln!(writer, "ERROR: {} ", e.msg)?;
            continue;
        }

        let program = result.unwrap();

        let evaluated = eval(program);

        if evaluated.is_err() {
            let e = evaluated.unwrap_err();
            writeln!(writer, "ERROR: {} ", e.msg)?;
            continue;
        }

        let r = evaluated.unwrap();

        writeln!(writer, "{}", r)?;
    }
    Ok(())
}

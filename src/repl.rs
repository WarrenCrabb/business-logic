use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

// Assuming you have modules named `lexer` and `token` defined elsewhere.
use crate::lexer::Lexer;
use crate::token::Token;

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
        let mut lex = Lexer::new(&line);
        loop {
            // Get the next token.
            let tok: Token = lex.next_token();
            // Print the token using Debug formatting.
            writeln!(writer, "{:?}", tok)?;

            // Stop if we reached the EOF token.
            if tok == Token::EOF {
                break;
            }
        }
    }
    Ok(())
}

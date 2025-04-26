use interpreter::{evaluator::Evaluator, parser::Parser};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn eval_business_logic(input: String) -> Result<String, String> {
    let mut parser = Parser::new_parser(input);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(err) => return Err(err.to_string()),
    };

    let mut evaluator = Evaluator::default();

    if let Err(err) = evaluator.eval_program(program) {
        return Err(err.to_string());
    }

    Ok(evaluator.output_buffer)
}

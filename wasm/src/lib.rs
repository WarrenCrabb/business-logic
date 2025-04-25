use interpreter::{evaluator::Evaluator, lexer::Lexer, object::environment::Env, parser::Parser};

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     // ...
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

// #[wasm_bindgen]
// pub fn greet() {
//     console_log!("Hello, console!");
// }

#[wasm_bindgen]
pub fn eval_business_logic(input: String) -> Result<String, String> {
    let env = Env::default();
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    // let mut parser = Parser::new_parser(input);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(err) => return Err(err.to_string()),
    };

    let mut evaluator = Evaluator::default();

    if let Err(err) = evaluator.eval(program, &env) {
        return Err(err.to_string());
    }

    Ok(evaluator.output_buffer)
}

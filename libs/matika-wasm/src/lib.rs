use matika_interpreter::{Interpreter, Parser, Scanner};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn interpret(txt: String) -> f64 {
    let mut scanner = Scanner::new(txt);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(stmts).into()
}

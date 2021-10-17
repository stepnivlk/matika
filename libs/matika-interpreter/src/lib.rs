mod callable;
mod environment;
mod expr;
mod functions;
mod interpreter;
mod parser;
mod plotter;
mod scanner;
mod stmt;
mod token;
mod value;
mod visitor;

pub use callable::Callable;
pub use expr::Expr;
pub use interpreter::Interpreter;
pub use parser::Parser;
pub use plotter::Plotter;
pub use scanner::Scanner;
pub use stmt::Stmt;
pub use value::Value;

pub struct Matika {
    interpreter: Interpreter,
}

impl Matika {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new()
        }
    }

    pub fn with_plotter(mut self, plotter: Box<dyn Plotter>) -> Self {
        self.interpreter = self.interpreter.with_plotter(plotter);

        self
    }

    pub fn eval(&mut self, txt: String) -> Value {
        let mut scanner = Scanner::new(txt);
        let tokens = scanner.scan();

        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        self.interpreter.interpret(stmts)
    }
}
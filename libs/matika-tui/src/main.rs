use matika_interpreter::{Interpreter, Parser, Plotter, Scanner, Stmt, Value};
use std::io::{stdin, stdout, Write};
use textplots::{Chart, Plot, Shape};

struct TuiPlotter;

impl Plotter for TuiPlotter {
    fn plot(&self, points: Vec<(f32, f32)>) -> Value {
        let ch = Chart::default()
            .lineplot(&Shape::Lines(&points[..]))
            .to_string();

        println!("{}", ch);

        Value::Number(0.0)
    }
}

struct Runner {
    interpreter: Interpreter,
}

impl Runner {
    fn new() -> Self {
        Self {
            interpreter: Interpreter::new().with_plotter(Box::new(TuiPlotter)),
        }
    }

    fn run_prompt(&mut self) {
        loop {
            let line = self.read_line();

            let stmts = self.parse_line(line);

            let result: f64 = self.interpreter.interpret(stmts).into();

            println!("{}", result);
        }
    }

    fn parse_line(&self, line: String) -> Vec<Stmt> {
        let mut scanner = Scanner::new(line);
        let tokens = scanner.scan();

        let mut parser = Parser::new(tokens);

        parser.parse()
    }

    fn read_line(&self) -> String {
        let mut line = String::new();

        print!(">>> ");

        let _ = stdout().flush();

        stdin().read_line(&mut line).unwrap();

        match line.chars().next_back() {
            Some('\n') => line.pop(),
            Some('\r') => line.pop(),
            _ => None,
        };

        line
    }
}

fn main() {
    let mut runner = Runner::new();

    runner.run_prompt();
}

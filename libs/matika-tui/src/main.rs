use matika_interpreter::{Plotter, Value, Matika};
use std::collections::HashSet;
use textplots::{Chart, Plot, Shape};
use rustyline::error::ReadlineError;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, Hint};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{Cmd, CompletionType, Config, Context, Editor, KeyEvent};
use rustyline_derive::{Completer, Helper, Highlighter, Validator};
use std::borrow::Cow::{self, Borrowed, Owned};

struct TuiPlotter;

impl Plotter for TuiPlotter {
    fn plot(&self, points: Vec<(f32, f32)>) -> Value {
        let chart = Chart::default()
            .lineplot(&Shape::Lines(&points[..]))
            .to_string();

        println!("{}", chart);

        Value::Number(0.0)
    }
}

#[derive(Completer, Helper, Validator, Highlighter)]
struct MatikaHinter {
    hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for MatikaHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                if hint.display.starts_with(line) {
                    Some(hint.suffix(pos))
                } else {
                    None
                }
            })
            .next()
    }
}

fn hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();

    set.insert(CommandHint::new("pi()", "pi()"));
    set.insert(CommandHint::new("sin()", "sin("));
    set.insert(CommandHint::new("factors()", "factors("));
    set.insert(CommandHint::new("plot()", "plot("));

    set
}


#[derive(Helper)]
struct MatikaHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: MatikaHinter,
    colored_prompt: String,
}

impl Completer for MatikaHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for MatikaHelper {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<CommandHint> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for MatikaHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for MatikaHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

struct EditorBuilder;

impl EditorBuilder {
    fn build() -> Editor<MatikaHelper> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .build();

        let hinter = MatikaHinter { hints: hints() };

        let h = MatikaHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter,
            colored_prompt: "".to_owned(),
            validator: MatchingBracketValidator::new(),
        };

        let mut rl = Editor::with_config(config);

        rl.set_helper(Some(h));

        rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
        rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);

        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        rl
    }
}

struct Runner {
    matika: Matika,
    editor: Editor<MatikaHelper>,
}

impl Runner {
    fn new() -> Self {
        Self {
            matika: Matika::new().with_plotter(Box::new(TuiPlotter)),
            editor: EditorBuilder::build(),
        }
    }

    fn run_prompt(&mut self) {
        if self.editor.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            match self.read_line() {
                Ok(line) => {
                    let result = self.matika.eval(line);

                    println!("{}", result);
                }
                Err(err) => {
                    println!("{}", err);
                    break
                }
            }
        }

        self.editor.save_history("history.txt").unwrap();
    }

    fn read_line(&mut self) -> Result<String, ReadlineError> {
        let prompt = ">>> ";

        self.editor.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", prompt);

        let readline = self.editor.readline(&prompt);

        match readline {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());

                Ok(line)
            },
            err @ Err(ReadlineError::Interrupted) => {
                err
            },
            err @ Err(ReadlineError::Eof) => {
                err
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}

fn main() {
    let mut runner = Runner::new();

    runner.run_prompt();
}

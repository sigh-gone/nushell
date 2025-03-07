use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    span, Category, Example, PipelineData, ShellError, Signature, Type, Value,
};
use std::io::IsTerminal as _;

#[derive(Clone)]
pub struct IsTerminal;

impl Command for IsTerminal {
    fn name(&self) -> &str {
        "is-terminal"
    }

    fn signature(&self) -> Signature {
        Signature::build("is-terminal")
            .input_output_type(Type::Nothing, Type::Bool)
            .switch("stdin", "Check if stdin is a terminal", Some('i'))
            .switch("stdout", "Check if stdout is a terminal", Some('o'))
            .switch("stderr", "Check if stderr is a terminal", Some('e'))
            .category(Category::Platform)
    }

    fn usage(&self) -> &str {
        "Check if stdin, stdout, or stderr is a terminal."
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: r#"Return "terminal attached" if standard input is attached to a terminal, and "no terminal" if not."#,
            example: r#"if (is-terminal --stdin) { "terminal attached" } else { "no terminal" }"#,
            result: Some(Value::test_string("terminal attached")),
        }]
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["input", "output", "stdin", "stdout", "stderr", "tty"]
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let stdin = call.has_flag("stdin");
        let stdout = call.has_flag("stdout");
        let stderr = call.has_flag("stderr");

        let is_terminal = match (stdin, stdout, stderr) {
            (true, false, false) => std::io::stdin().is_terminal(),
            (false, true, false) => std::io::stdout().is_terminal(),
            (false, false, true) => std::io::stderr().is_terminal(),
            (false, false, false) => {
                return Err(ShellError::MissingParameter {
                    param_name: "one of --stdin, --stdout, --stderr".into(),
                    span: call.head,
                });
            }
            _ => {
                let spans: Vec<_> = call.arguments.iter().map(|arg| arg.span()).collect();
                let span = span(&spans);

                return Err(ShellError::IncompatibleParametersSingle {
                    msg: "Only one stream may be checked".into(),
                    span,
                });
            }
        };

        Ok(PipelineData::Value(
            Value::bool(is_terminal, call.head),
            None,
        ))
    }
}

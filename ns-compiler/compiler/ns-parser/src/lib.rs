mod decl;
mod expr;
mod ident;
mod module;
mod package;
mod parser;
mod patterns;
mod shared;
mod stmt;
mod type_node;

pub use package::{main_parse, main_parse_with_diagnostics};
pub use parser::Parser;

use ns_ast::Module;
use ns_diagnostics::{Diagnostic, SourceSpan};
use ns_lexer::lexer;

pub type ParseResult<T> = Result<T, Vec<Diagnostic>>;

pub fn parse_module_with_diagnostics(path: &str) -> ParseResult<Module> {
    let tokens = lexer(path);
    let mut parser = Parser::new_with_path(tokens, Some(path.to_string()));
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parser.parse_module()));
    std::panic::set_hook(prev_hook);
    let mut parser_diags = parser.take_diagnostics();
    match result {
        Ok(module) => {
            if parser_diags.is_empty() {
                Ok(module)
            } else {
                Err(parser_diags)
            }
        }
        Err(payload) => {
            parser_diags.push(panic_payload_to_syntax_diagnostic(&payload));
            Err(parser_diags)
        }
    }
}

pub(crate) fn panic_payload_to_syntax_diagnostic(
    payload: &Box<dyn std::any::Any + Send>,
) -> Diagnostic {
    if let Some(parse) = payload.downcast_ref::<parser::ParsePanic>() {
        return Diagnostic::error_with_code("P1000", parse.message.clone())
            .with_span(SourceSpan {
                file: parse.file.clone(),
                line: None,
                column: None,
                start: Some(parse.range.start),
                end: Some(parse.range.end),
                label: Some("syntax error".to_string()),
            })
            .with_note("parser expects syntactically valid token sequence");
    }

    if let Some(msg) = payload.downcast_ref::<String>() {
        return Diagnostic::error_with_code("P1000", msg.clone());
    }
    if let Some(msg) = payload.downcast_ref::<&str>() {
        return Diagnostic::error_with_code("P1000", (*msg).to_string());
    }
    Diagnostic::error_with_code("P1000", "unknown parser panic")
}

use ns_diagnostics::Diagnostic;

pub type SirResult<T> = Result<T, Vec<Diagnostic>>;

pub fn sir_error(code: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic::error_with_code(code, message.into())
}

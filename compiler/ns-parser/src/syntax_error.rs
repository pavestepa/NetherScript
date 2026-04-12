use ns_lexer::TextRange;


#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub range: TextRange,
}
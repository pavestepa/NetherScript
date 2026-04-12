use crate::syntax::text_range::TextRange;

#[derive(Debug, Clone)]
pub struct SyntaxError {
    pub message: String,
    pub range: TextRange,
}

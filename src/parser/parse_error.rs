use crate::TextRange;

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
    MissingToken,
    ExpectedExpression,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub range: TextRange,
}

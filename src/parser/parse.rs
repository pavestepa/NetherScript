use crate::parser::SyntaxError;

#[derive(Debug)]
pub struct Parse<T> {
    pub syntax: T,
    pub errors: Vec<SyntaxError>,
}
pub fn parse<T>(syntax: T, errors: Vec<SyntaxError>) -> Parse<T> {
    Parse { syntax, errors }
}

use crate::parser::SyntaxError;

pub struct Parse<T> {
    pub syntax: T,
    pub errors: Vec<SyntaxError>,
}

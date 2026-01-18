#[derive(Debug)]
pub enum Ast<T> {
    Parsed(T),
    Error,
}

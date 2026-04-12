#[derive(Debug, Clone)]
pub enum Ast<T> {
    Parsed(T),
    Error(String),
}

impl<T> Ast<T> {
    pub fn err<I: Into<String>>(err: I) -> Self {
        Ast::Error(err.into())
    }

    pub fn unwrap(self) -> T {
        match self {
            Ast::Parsed(value) => value,
            Ast::Error(err) => panic!("called `Ast::unwrap()` on an `Error` value: {}", err),
        }
    }
}

use ns_atom::Atom;

/// Fixed primitive value carried inside a literal expression.
#[derive(Debug, Clone)]
pub enum LiteralExpr {
    Number(Atom),
    String(Atom),
    Boolean(Atom),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Type,
    Value,
    Function,
    Interface,
    Class,
    Enum,
    TypeParam,
    Field,
    Method,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: String,
    pub kind: SymbolKind,
}

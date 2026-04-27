#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub u32);

#[derive(Debug, Clone)]
pub enum Type {
    Error,
    Named { name: String, args: Vec<TypeId> },
    Dynamic { interface: String },
    TypeParam { name: String },
}

use crate::types::TypeId;

#[derive(Clone, Debug)]
pub(super) enum CheckedType {
    Resolved(TypeId),
    Callable { params: Vec<TypeId>, ret: TypeId },
    Error,
}

mod binding_ptrn;
mod typed_binding_ptrn;

pub use binding_ptrn::BindingPtrn;
pub use typed_binding_ptrn::TypedBindingPtrn;

#[derive(Debug, Clone)]
pub enum Ptrn {
    Binding,
    TypedBinding,
}

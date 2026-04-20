mod export;
mod import;

pub use export::Export;
pub use import::Import;

use crate::{Ident, decl::Decl};

#[derive(Debug)]
pub struct Module {
    decls: Vec<Decl>,
    exports: Vec<Export>,
    imports: Vec<Import>,
    index: Vec<Ident>,
}

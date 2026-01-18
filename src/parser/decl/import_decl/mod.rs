use crate::{
    ast::ImportDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_import_decl(&mut self) -> ImportDecl {
        parse(ImportDecl { from: vec![] }, error)
    }
}

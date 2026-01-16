use crate::{
    ast::ExportDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_export_decl(&mut self) -> Parse<ExportDecl> {
        parse(ExportDecl {}, errors)
    }
}

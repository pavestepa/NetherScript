use crate::{
    ast::IndexDecl,
    parser::{parse::parse, Parse, Parser},
};

impl Parser {
    pub fn parse_index_decl(&mut self) -> IndexDecl {
        parse(IndexDecl {}, error)
    }
}

use crate::{ast::Module, parser::Parser};

impl Parser {
    pub fn parse_module(&mut self) -> Module {
        let mut decls = vec![];

        println!("started parse of module");
        while self.is_not_end() {
            decls.push(self.parse_decl());
        }

        println!("ended parse of module");

        return Module::new(decls);
    }
}

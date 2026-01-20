use crate::{ast::Module, parser::Parser};

impl Parser {
    pub fn parse_module(&mut self) -> Module {
        let mut decls = vec![];

        println!("started parse of module");
        while self.peek().is_some() {
            decls.push(self.parse_decl());
            self.next();
        }

        println!("ended parse of module");

        return Module::new(decls);
    }
}

use crate::{ast::Module, parser::Parser};

impl Parser {
    pub fn parse_module(&mut self) -> Module {
        println!("[STARTED] parse Module");
        let mut decls = vec![];

        while self.is_not_end() {
            decls.push(self.parse_decl());
        }

        println!("ended parse of module");

        return Module::new(decls);
    }
}

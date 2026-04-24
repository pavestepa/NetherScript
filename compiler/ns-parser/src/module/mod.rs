use ns_ast::Module;
use ns_lexer::{Keyword, TokenKind};

use crate::Parser;

mod export;
mod index;
mod import;

impl Parser {
    pub fn parse_module(&mut self) -> Module {
        let mut decls = vec![];
        let mut exports = vec![];
        let mut imports = vec![];
        let mut index = vec![];

        while self.is_not_end() {
            self.consume_newlines();
            if !self.is_not_end() {
                break;
            }
            if let TokenKind::Keyword(keyword) = self.current().kind {
                match keyword {
                    Keyword::Import => {
                        self.parse(TokenKind::Keyword(Keyword::Import));
                        imports.push(self.parse_import());
                        continue;
                    }
                    Keyword::Export => {
                        self.parse(TokenKind::Keyword(Keyword::Export));
                        if matches!(
                            self.current().kind,
                            TokenKind::Keyword(
                                Keyword::Class
                                    | Keyword::Function
                                    | Keyword::Enum
                                    | Keyword::Interface
                                    | Keyword::Const
                                    | Keyword::Type
                            )
                        ) {
                            let decl = self.parse_export_decl_form();
                            decls.push(ns_ast::ModuleDecl::new(decl, true));
                        } else {
                            exports.push(self.parse_export());
                        }
                        continue;
                    }
                    Keyword::Index => {
                        self.parse(TokenKind::Keyword(Keyword::Index));
                        index.push(self.parse_index());
                        continue;
                    }
                    _ => {}
                }
            }

            decls.push(ns_ast::ModuleDecl::new(self.parse_decl(), false));
        }

        Module::new_full(decls, exports, imports, index)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

    use ns_ast::{Decl, Export};
    use ns_lexer::lexer;

    use crate::Parser;

    fn parse_module_from_source(source: &str) -> ns_ast::Module {
        let mut path = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        path.push(format!("ns_parser_export_test_{}.ns", nanos));
        write_temp(&path, source);
        let tokens = lexer(path.to_str().expect("utf8 temp path"));
        let mut parser = Parser::new(tokens);
        let module = parser.parse_module();
        let _ = fs::remove_file(path);
        module
    }

    fn write_temp(path: &PathBuf, source: &str) {
        fs::write(path, source).expect("write temp source file");
    }

    #[test]
    fn export_ident_is_export_ident_variant() {
        let module = parse_module_from_source("export Foo;");
        assert_eq!(module.exports().len(), 1);
        match &module.exports()[0] {
            Export::Ident(_) => {}
            other => panic!("expected Export::Ident, got {:?}", other),
        }
    }

    #[test]
    fn export_braced_is_export_idents_variant() {
        let module = parse_module_from_source("export {Foo};");
        assert_eq!(module.exports().len(), 1);
        match &module.exports()[0] {
            Export::Idents(idents) => assert_eq!(idents.len(), 1),
            other => panic!("expected Export::Idents, got {:?}", other),
        }
    }

    #[test]
    fn export_decl_marked_exported_in_decls_without_export_entry() {
        let module = parse_module_from_source("export class Foo {}");
        assert_eq!(module.exports().len(), 0);
        assert!(
            module
                .decls()
                .iter()
                .any(|d| d.exported() && matches!(d.decl(), Decl::Class(_))),
            "expected exported Decl::Class in module.decls"
        );
        let dbg = format!("{:?}", module);
        assert!(dbg.contains("Foo"), "expected parsed module to include class Foo");
    }
}

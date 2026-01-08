use crate::{
    ast::{shared::VarKind, stmt::VarStmt},
    lexer::{Keyword, Token},
    parser::Parser,
};

impl Parser {
    pub fn parse_var_stmt(&mut self) -> Result<VarStmt, String> {
        println!("var_stmt.");
        // TODO: to refactor
        // Check to "let" or "const"
        let kind = match self.keyword() {
            Ok(Keyword::Let) => VarKind::Let,
            Ok(Keyword::Const) => VarKind::Const,
            Err(e) => {
                return Err(e);
            }
            e => {
                return Err(format!("expected Let or Const, but found {:?}", e.unwrap()));
            }
        };
        println!("[kind]: {:?}", kind);
        self.next();

        // Check name of variable
        let name = self.ident()?;
        println!("[name]: {:?}", name);
        self.next();

        // Check initial value;
        let init;

        // Check if variablie is not initiated
        if self.is(Token::Semicolon) {
            init = None
        } else {
            // Check to "="
            if self.is_not(Token::Assign) {
                return Err(format!(
                    "expected Let or Assign, but found {:?}",
                    self.peek().unwrap()
                ));
            }
            self.next();

            init = Some(Box::new(self.parse_expr(0)?));
        }
        let var_stmt = VarStmt {
            kind: kind,
            name: name,
            init: init,
        };
        println!("{:?}", var_stmt);
        Ok(var_stmt)
    }
}

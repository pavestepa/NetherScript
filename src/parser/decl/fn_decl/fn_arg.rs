use crate::{
    ast::{decl::FnArg, Typ},
    atom,
    lexer::Token,
    parser::Parser,
};

pub fn parse_fn_arg(parser: &mut Parser) -> Result<FnArg, String> {
    let ident;
    let typ;
    if let Token::Ident(value) = parser.peek().unwrap() {
        ident = *value;
        parser.next();
    } else {
        return expect_err(Token::Ident(atom("".to_string())), parser);
    }
    if parser.peek().unwrap() == &Token::Colon {
        parser.next();
    } else {
        return expect_err(Token::Colon, parser);
    }
    typ = parser.parse_type();
    if typ.is_err() {
        return Err(typ.err().unwrap());
    }
    parser.next();
    return Ok(FnArg::new(ident, typ.unwrap()));
}

fn expect_err(token: Token, parser: &Parser) -> Result<FnArg, String> {
    Err(format!(
        "expected {:?} but found {:?}",
        token,
        parser.peek().unwrap()
    ))
}

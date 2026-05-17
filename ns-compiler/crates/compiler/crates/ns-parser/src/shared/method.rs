use ns_ast::{Callable, Method, NamedType, This, ThisReceiver, TypeNode, TypedBinding};
use ns_lexer::{Keyword, TokenKind};
use ns_atom::atom;
use crate::Parser;

impl Parser {
    pub fn parse_method(&mut self) -> Method {
        let signature = self.parse_callable_signature(true);
        let body = self.parse_stmts_block();

        Method { signature, body }
    }

    pub fn parse_callable_signature(&mut self, allow_this_receiver: bool) -> Callable {
        let ident = self.parse_ident();
        let type_parameters = self.parse_type_parameters_in_angle_brackets();
        let (this, arguments) = self.parse_arguments_with_optional_this(allow_this_receiver);
        let return_type = if self.current().kind == TokenKind::Colon {
            self.parse(TokenKind::Colon);
            self.parse_type_node()
        } else {
            TypeNode::Named(NamedType::simple(ns_ast::Ident::new(atom("void"))))
        };

        Callable {
            ident,
            type_parameters,
            this,
            arguments,
            return_type,
        }
    }

    fn parse_arguments_with_optional_this(
        &mut self,
        allow_this_receiver: bool,
    ) -> (This, Vec<TypedBinding>) {
        self.parse(TokenKind::LeftParen);
        self.consume_newlines();

        let this = if allow_this_receiver && self.starts_this_receiver() {
            self.parse_this()
        } else {
            This::Static
        };
        self.consume_newlines();

        let mut args = Vec::new();
        if allow_this_receiver
            && matches!(this, This::Receiver(_))
            && self.current().kind == TokenKind::Comma
        {
            self.parse(TokenKind::Comma);
            self.consume_newlines();
        }

        while self.current().kind != TokenKind::RightParen {
            let ident = self.parse_ident();
            self.parse(TokenKind::Colon);
            let type_ref = self.parse_type_node();
            args.push(TypedBinding { ident, type_ref });

            if self.current().kind == TokenKind::Comma {
                self.parse(TokenKind::Comma);
                self.consume_newlines();
            } else {
                break;
            }
        }

        self.parse(TokenKind::RightParen);
        (this, args)
    }

    fn starts_this_receiver(&self) -> bool {
        if self.current().kind == TokenKind::Ident(atom("this")) {
            return true;
        }
        if self.current().kind == TokenKind::Ampersand
            && self.peek(1).kind == TokenKind::Ident(atom("this"))
        {
            return true;
        }
        if self.current().kind == TokenKind::Ampersand
            && self.peek(1).kind == TokenKind::Keyword(Keyword::Mut)
            && self.peek(2).kind == TokenKind::Ident(atom("this"))
        {
            return true;
        }
        false
    }

    fn parse_this(&mut self) -> This {
        if self.current().kind == TokenKind::Ident(atom("this")) {
            self.parse(TokenKind::Ident(atom("this")));
            let type_annotation = if self.current().kind == TokenKind::Colon {
                self.parse(TokenKind::Colon);
                Some(self.parse_type_node())
            } else {
                None
            };
            return This::Receiver(ThisReceiver {
                ref_kind: ns_ast::RefKind::Own,
                type_annotation,
            });
        }

        if self.current().kind == TokenKind::Ampersand {
            self.parse(TokenKind::Ampersand);
            let ref_kind = if self.current().kind == TokenKind::Keyword(Keyword::Mut) {
                self.parse(TokenKind::Keyword(Keyword::Mut));
                ns_ast::RefKind::Mut
            } else {
                ns_ast::RefKind::Ref
            };
            self.parse(TokenKind::Ident(atom("this")));
            let type_annotation = if self.current().kind == TokenKind::Colon {
                self.parse(TokenKind::Colon);
                Some(self.parse_type_node())
            } else {
                None
            };
            return This::Receiver(ThisReceiver {
                ref_kind,
                type_annotation,
            });
        }

        if self.current().kind == TokenKind::RightParen {
            return This::Static;
        }

        self.panic_at_current("expected `this`, `&this`, `&mut this`, or `)` in method parameters")
    }
}

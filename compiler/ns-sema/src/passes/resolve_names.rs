use ns_ast::{
    AssignTarget, Binding, Callable, ClassDecl, ConstDecl, Decl, EnumDecl, Expr, FunctionDecl, Ident,
    InterfaceDecl, Module, ModuleDecl, Stmt, StmtsBlock, This, TypeDecl, TypeModifierDecl, TypeNode,
    TypeParameter,
};

use crate::{
    context::SemaContext,
    symbol::SymbolKind,
};

pub fn run(ctx: &mut SemaContext, module: &Module) {
    for module_decl in module.decls() {
        resolve_module_decl(ctx, module_decl);
    }
}

fn resolve_module_decl(ctx: &mut SemaContext, module_decl: &ModuleDecl) {
    resolve_decl(ctx, module_decl.decl());
}

fn resolve_decl(ctx: &mut SemaContext, decl: &Decl) {
    match decl {
        Decl::Class(d) => resolve_class_decl(ctx, d),
        Decl::Const(d) => resolve_const_decl(ctx, d),
        Decl::Error(_) => {}
        Decl::Enum(d) => resolve_enum_decl(ctx, d),
        Decl::Function(d) => resolve_function_decl(ctx, d),
        Decl::Interface(d) => resolve_interface_decl(ctx, d),
        Decl::Type(d) => resolve_type_decl(ctx, d),
        Decl::TypeModifier(d) => resolve_type_modifier_decl(ctx, d),
    }
}

fn resolve_class_decl(ctx: &mut SemaContext, class_decl: &ClassDecl) {
    ctx.push_scope();
    resolve_type_parameters(ctx, &class_decl.type_parameters);

    if let Some(extends) = &class_decl.extends {
        resolve_type_ident(ctx, extends);
    }
    if let Some(implements) = &class_decl.implements {
        for interface in implements {
            resolve_interface_ident(ctx, interface);
        }
    }

    for field in &class_decl.fields {
        resolve_binding_type(ctx, &field.binding);
        if let Some(init) = &field.init {
            resolve_expr(ctx, init);
        }
    }

    for method in &class_decl.methods {
        resolve_callable_body(ctx, &method.signature, &method.body, ReceiverPolicy::Method);
    }

    ctx.pop_scope();
}

fn resolve_const_decl(ctx: &mut SemaContext, const_decl: &ConstDecl) {
    resolve_type_node(ctx, &const_decl.binding.type_ref);
    resolve_expr(ctx, &const_decl.val);
}

fn resolve_enum_decl(ctx: &mut SemaContext, enum_decl: &EnumDecl) {
    ctx.push_scope();
    resolve_type_parameters(ctx, &enum_decl.type_parameters);

    if let Some(implements) = &enum_decl.implements {
        for interface in implements {
            resolve_interface_ident(ctx, interface);
        }
    }

    for member in &enum_decl.members {
        if let Some(contains) = &member.contains {
            resolve_type_node(ctx, contains);
        }
    }

    for method in &enum_decl.methods {
        resolve_callable_body(ctx, &method.signature, &method.body, ReceiverPolicy::Method);
    }

    ctx.pop_scope();
}

fn resolve_function_decl(ctx: &mut SemaContext, function_decl: &FunctionDecl) {
    resolve_callable_body(
        ctx,
        &function_decl.signature,
        &function_decl.body,
        ReceiverPolicy::StaticOnly,
    );
}

fn resolve_interface_decl(ctx: &mut SemaContext, interface_decl: &InterfaceDecl) {
    ctx.push_scope();
    resolve_type_parameters(ctx, &interface_decl.type_parameters);

    for (signature, maybe_body) in &interface_decl.methods {
        resolve_callable_signature(ctx, signature, ReceiverPolicy::Method);

        if let Some(body) = maybe_body {
            resolve_callable_scope(
                ctx,
                signature,
                ReceiverPolicy::Method,
                |ctx| resolve_block_statements(ctx, body),
            );
        }
    }

    ctx.pop_scope();
}

fn resolve_type_decl(ctx: &mut SemaContext, type_decl: &TypeDecl) {
    ctx.push_scope();
    resolve_type_parameters(ctx, &type_decl.type_parameters);
    resolve_type_node(ctx, &type_decl.assign);
    ctx.pop_scope();
}

fn resolve_type_modifier_decl(ctx: &mut SemaContext, type_modifier_decl: &TypeModifierDecl) {
    match type_modifier_decl {
        TypeModifierDecl::Extends(d) => {
            resolve_type_node(ctx, &d.type_identifier);
            for (signature, body) in &d.methods {
                resolve_callable_body(ctx, signature, body, ReceiverPolicy::RequireReceiver);
            }
        }
        TypeModifierDecl::Implements(d) => {
            resolve_type_node(ctx, &d.type_identifier);
            for interface in &d.interfaces {
                resolve_type_node(ctx, interface);
            }
            for (signature, body) in &d.methods {
                resolve_callable_body(ctx, signature, body, ReceiverPolicy::RequireReceiver);
            }
        }
    }
}

fn resolve_type_parameters(ctx: &mut SemaContext, type_parameters: &[TypeParameter]) {
    for tp in type_parameters {
        let name = ident_name(&tp.ident);
        let sid = ctx.intern_symbol(name.clone(), SymbolKind::TypeParam);
        if !ctx.declare_type_in_current_scope(&name, sid) {
            ctx.error(format!("duplicate type parameter: {name}"));
        }
    }

    for tp in type_parameters {
        for bound in &tp.implements {
            resolve_interface_ident(ctx, bound);
        }
        if let Some(default_type) = &tp.default_type {
            resolve_type_node(ctx, default_type);
        }
    }
}

fn resolve_callable_body(
    ctx: &mut SemaContext,
    callable: &Callable,
    body: &StmtsBlock,
    receiver_policy: ReceiverPolicy,
) {
    resolve_callable_signature(ctx, callable, receiver_policy);
    resolve_callable_scope(ctx, callable, receiver_policy, |ctx| {
        resolve_block_statements(ctx, body);
    });
}

fn resolve_callable_scope(
    ctx: &mut SemaContext,
    callable: &Callable,
    receiver_policy: ReceiverPolicy,
    f: impl FnOnce(&mut SemaContext),
) {
    ctx.push_scope();

    resolve_type_parameters(ctx, &callable.type_parameters);

    if let This::Receiver(receiver) = &callable.this {
        if let Some(type_annotation) = &receiver.type_annotation {
            resolve_type_node(ctx, type_annotation);
        }
    }

    for argument in &callable.arguments {
        resolve_type_node(ctx, &argument.type_ref);
        let arg_name = ident_name(&argument.ident);
        let sid = ctx.intern_symbol(arg_name.clone(), SymbolKind::Value);
        if !ctx.declare_value_in_current_scope(&arg_name, sid) {
            ctx.error(format!("duplicate parameter: {arg_name}"));
        }
    }

    validate_receiver_policy(ctx, callable, receiver_policy);
    f(ctx);
    ctx.pop_scope();
}

fn resolve_callable_signature(
    ctx: &mut SemaContext,
    callable: &Callable,
    receiver_policy: ReceiverPolicy,
) {
    ctx.push_scope();
    resolve_type_parameters(ctx, &callable.type_parameters);

    if let This::Receiver(receiver) = &callable.this {
        if let Some(type_annotation) = &receiver.type_annotation {
            resolve_type_node(ctx, type_annotation);
        }
    }

    for argument in &callable.arguments {
        resolve_type_node(ctx, &argument.type_ref);
    }
    resolve_type_node(ctx, &callable.return_type);
    validate_receiver_policy(ctx, callable, receiver_policy);
    ctx.pop_scope();
}

fn validate_receiver_policy(ctx: &mut SemaContext, callable: &Callable, receiver_policy: ReceiverPolicy) {
    match (receiver_policy, &callable.this) {
        (ReceiverPolicy::StaticOnly, This::Receiver(_)) => {
            let name = ident_name(&callable.ident);
            ctx.error(format!("function `{name}` cannot have `this` receiver"));
        }
        (ReceiverPolicy::RequireReceiver, This::Static) => {
            let name = ident_name(&callable.ident);
            ctx.error(format!("method `{name}` in type modifier must declare `this` receiver"));
        }
        _ => {}
    }
}

fn resolve_block_statements(ctx: &mut SemaContext, block: &StmtsBlock) {
    for stmt in &block.stmts {
        resolve_stmt(ctx, stmt);
    }
}

fn resolve_stmt(ctx: &mut SemaContext, stmt: &Stmt) {
    match stmt {
        Stmt::Assign(s) => {
            match &s.target {
                AssignTarget::Ident(ident) => {
                    resolve_value_ident(ctx, ident);
                }
                AssignTarget::Member(_member) => {
                    // MemberExpr internals are not exposed by ns-ast; cannot descend further here.
                }
            }
            resolve_expr(ctx, &s.assign);
        }
        Stmt::Binding(s) => {
            resolve_binding_type(ctx, &s.binding);
            if let Some(value) = &s.value {
                resolve_expr(ctx, value);
            }

            let name = ident_name(&s.binding.ident);
            let sid = ctx.intern_symbol(name.clone(), SymbolKind::Value);
            if !ctx.declare_value_in_current_scope(&name, sid) {
                ctx.error(format!("duplicate local binding: {name}"));
            }
        }
        Stmt::Error(_) => {}
        Stmt::Break(_) => {}
        Stmt::Expr(s) => resolve_expr(ctx, &s.expr),
        Stmt::If(s) => {
            resolve_expr(ctx, &s.test);

            ctx.push_scope();
            resolve_block_statements(ctx, &s.body);
            ctx.pop_scope();

            if let Some(alt) = &s.alt {
                ctx.push_scope();
                resolve_block_statements(ctx, alt);
                ctx.pop_scope();
            }
        }
        Stmt::Loop(s) => {
            ctx.push_scope();
            resolve_block_statements(ctx, &s.body);
            ctx.pop_scope();
        }
        Stmt::Return(s) => {
            if let Some(arg) = &s.arg {
                resolve_expr(ctx, arg);
            }
        }
        Stmt::While(s) => {
            resolve_expr(ctx, &s.test);
            ctx.push_scope();
            resolve_block_statements(ctx, &s.body);
            ctx.pop_scope();
        }
    }
}

fn resolve_expr(ctx: &mut SemaContext, expr: &Expr) {
    match expr {
        Expr::BindingExpr(e) => resolve_value_ident(ctx, &e.0),
        Expr::CallExpr(e) => {
            resolve_expr(ctx, &e.callee);
            for type_arg in &e.type_arguments {
                resolve_type_node(ctx, type_arg);
            }
            for arg in &e.args {
                resolve_expr(ctx, arg);
            }
        }
        Expr::Error(_) => {}
        Expr::LiteralExpr(_) => {}
        Expr::MemberExpr(_e) => {
            // MemberExpr internals are private in ns-ast; traverse what is exposed elsewhere.
        }
        Expr::BinaryExpr(e) => {
            resolve_expr(ctx, &e.left);
            resolve_expr(ctx, &e.right);
        }
        Expr::LogicalExpr(e) => {
            resolve_expr(ctx, &e.left);
            resolve_expr(ctx, &e.right);
        }
        Expr::Referencing(e) => resolve_expr(ctx, &e.expr),
        Expr::UnaryExpr(e) => match e {
            ns_ast::UnaryExpr::Minus(v)
            | ns_ast::UnaryExpr::Plus(v)
            | ns_ast::UnaryExpr::Not(v)
            | ns_ast::UnaryExpr::BitNot(v)
            | ns_ast::UnaryExpr::Deref(v) => resolve_expr(ctx, v),
        },
        Expr::NewExpr(e) => {
            resolve_type_ident(ctx, &e.class_ident);
            for arg in &e.args {
                resolve_expr(ctx, arg);
            }
        }
        Expr::StructLiteral(e) => {
            resolve_type_ident(ctx, &e.struct_name);
            for field in &e.fields {
                resolve_expr(ctx, &field.value);
            }
        }
        Expr::TemplateString(e) => {
            for part in &e.parts {
                if let ns_ast::TemplateStringPart::Expr(expr) = part {
                    resolve_expr(ctx, expr);
                }
            }
        }
    }
}

fn resolve_binding_type(ctx: &mut SemaContext, binding: &Binding) {
    if let Some(type_ref) = &binding.type_ref {
        resolve_type_node(ctx, type_ref);
    }
}

fn resolve_type_node(ctx: &mut SemaContext, type_node: &TypeNode) {
    match type_node {
        TypeNode::Named(named) => {
            resolve_type_ident(ctx, &named.ident);
            for arg in &named.type_arguments {
                resolve_type_node(ctx, arg);
            }
        }
        TypeNode::Dynamic(dynamic) => {
            resolve_interface_ident(ctx, &dynamic.interface);
        }
    }
}

fn resolve_value_ident(ctx: &mut SemaContext, ident: &Ident) {
    let name = ident_name(ident);
    if ctx.resolve_value_name(&name).is_none() {
        ctx.error(format!("unresolved value: {name}"));
    }
}

fn resolve_type_ident(ctx: &mut SemaContext, ident: &Ident) {
    let name = ident_name(ident);
    if ctx.resolve_type_name(&name).is_none() {
        ctx.error(format!("unresolved type: {name}"));
    }
}

fn resolve_interface_ident(ctx: &mut SemaContext, ident: &Ident) {
    let name = ident_name(ident);
    if ctx.resolve_type_name(&name).is_none() {
        ctx.error(format!("unresolved interface: {name}"));
    }
}

fn ident_name(ident: &Ident) -> String {
    ident.clone().into_simple().as_str().to_string()
}

#[derive(Clone, Copy)]
enum ReceiverPolicy {
    StaticOnly,
    Method,
    RequireReceiver,
}

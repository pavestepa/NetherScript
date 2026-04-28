use ns_ast::ClassDecl;

use crate::ast_parser::Lowerer;
use crate::ast_parser::shared::method::lower_class_method;
use crate::decl::SirDecl;
use crate::diagnostics::SirResult;
use crate::ids::SirFieldId;
use crate::symbols::SirFieldSymbol;

pub(crate) fn lower(lowerer: &mut Lowerer, c: &ClassDecl) -> SirResult<()> {
    let class_name = c.ident.clone().into_simple().as_str().to_string();
    let Some(class_id) = lowerer.class_by_name.get(&class_name).copied() else {
        return Err(crate::ast_parser::missing_predeclared_class_err(&class_name));
    };

    let inherited_class = c.extends.as_ref().and_then(|p| {
        let parent_name = p.clone().into_simple().as_str().to_string();
        lowerer.class_by_name.get(&parent_name).copied()
    });

    let mut field_ids = Vec::new();
    for field in &c.fields {
        let field_name = field.binding.ident.clone().into_simple().as_str().to_string();
        let field_ty = field.binding.type_ref.as_ref().map(|t| lowerer.lower_type(t));
        let fid = SirFieldId(lowerer.next_field_id);
        lowerer.next_field_id += 1;
        lowerer.program.fields.push(SirFieldSymbol {
            id: fid,
            owner: class_id,
            name: field_name.clone(),
            ty: field_ty,
        });
        field_ids.push(fid);
    }

    let mut method_ids = Vec::new();
    let mut constructor = None;
    for method in &c.methods {
        let method_name = method.signature.ident.clone().into_simple().as_str().to_string();
        let fn_id = lower_class_method(
            lowerer,
            class_id,
            &method_name,
            &method.signature.arguments,
            &method.signature.return_type,
            &method.body.stmts,
        );
        if method_name == "constructor" {
            constructor = Some(fn_id);
        } else {
            method_ids.push(fn_id);
        }
    }

    if let Some(class_symbol) = lowerer.program.classes.iter_mut().find(|cls| cls.id == class_id) {
        class_symbol.fields = field_ids.clone();
        class_symbol.methods = method_ids.clone();
        class_symbol.inherited_class = inherited_class;
    }

    lowerer.program.decls.push(SirDecl::Class {
        id: class_id,
        fields: field_ids,
        methods: method_ids,
        constructor,
    });
    Ok(())
}

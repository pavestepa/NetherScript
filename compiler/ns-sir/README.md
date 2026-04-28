# ns-sir

`ns-sir` is a semantic intermediate representation layer between AST and later semantic/codegen passes.

## Current status

This is PR1 + partial PR2:

- Core stable IDs (`SirTypeId`, `SirValueId`, `SirFnId`, `SirClassId`, `SirFieldId`, `SirTraitId`)
- Base SIR nodes for declarations, statements, expressions, symbols, and types
- AST -> SIR lowering for:
  - `const`
  - `function`
  - `type` aliases
  - basic statements/expressions
- SIR verifier (basic symbol ID invariants)
- SIR dump support for snapshot-style tests

## AST -> SIR mapping (implemented subset)

- `Decl::Const` -> `SirDecl::Const`
- `Decl::Function` -> `SirDecl::Function`
- `Decl::Type` -> `SirDecl::TypeAlias`
- `Decl::Error` and unsupported decls -> `SirDecl::Error` (temporary)

- `Stmt::Binding` -> `SirStmt::Let`
- `Stmt::Assign` -> `SirStmt::Assign`
- `Stmt::Expr` -> `SirStmt::Expr`
- `Stmt::Return` -> `SirStmt::Return`
- `Stmt::If` -> `SirStmt::If`
- `Stmt::Loop` -> `SirStmt::Loop`
- `Stmt::While` -> `SirStmt::While`
- `Stmt::Break` -> `SirStmt::Break`
- `Stmt::Error` -> `SirStmt::Error`

- `Expr::BindingExpr` -> `SirExpr::ValueRef` (or `SirExpr::Error` if unresolved at lowering scope)
- `Expr::CallExpr` -> `SirExpr::Call`
- `Expr::LiteralExpr` -> `SirExpr::Literal`
- `Expr::UnaryExpr` -> `SirExpr::Unary`
- `Expr::BinaryExpr` / `Expr::LogicalExpr` -> `SirExpr::Binary`
- `Expr::MemberExpr` -> `SirExpr::Member`
- `Expr::StructLiteral` -> `SirExpr::StructLiteral`
- `Expr::NewExpr` -> `SirExpr::New`
- `Expr::Error` -> `SirExpr::Error`

## Planned next steps

- PR3: class + inheritance desugaring with `inherited_class`
- PR4: constructor normalization and init-order checks
- PR5: import/export visibility and symbol linking in SIR
- PR6: migrate selected sema checks to run on SIR

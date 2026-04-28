use crate::expr::SirExpr;
use crate::ids::{SirClassId, SirEnumId, SirFieldId, SirFnId, SirTypeId, SirValueId};
use crate::stmt::SirStmt;
use crate::symbols::{
    SirClassSymbol, SirEnumSymbol, SirExportLink, SirFieldSymbol, SirFunctionSymbol, SirImportLink,
    SirTraitSymbol, SirValueSymbol,
};

#[derive(Debug, Clone)]
pub struct SirProgram {
    pub decls: Vec<SirDecl>,
    pub values: Vec<SirValueSymbol>,
    pub functions: Vec<SirFunctionSymbol>,
    pub classes: Vec<SirClassSymbol>,
    pub enums: Vec<SirEnumSymbol>,
    pub fields: Vec<SirFieldSymbol>,
    pub traits: Vec<SirTraitSymbol>,
    pub imports: Vec<SirImportLink>,
    pub exports: Vec<SirExportLink>,
    pub types: Vec<crate::types::SirType>,
}

#[derive(Debug, Clone)]
pub enum SirDecl {
    Error,
    Const {
        value: SirValueId,
        expr: SirExpr,
    },
    Function {
        id: SirFnId,
        body: Vec<SirStmt>,
    },
    TypeAlias {
        name: String,
        ty: SirTypeId,
    },
    Class {
        id: SirClassId,
        fields: Vec<SirFieldId>,
        methods: Vec<SirFnId>,
        constructor: Option<SirFnId>,
    },
    Enum {
        id: SirEnumId,
    },
    Impl {
        target: SirImplTarget,
        methods: Vec<SirFnId>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SirImplTarget {
    Class(SirClassId),
    Enum(SirEnumId),
}

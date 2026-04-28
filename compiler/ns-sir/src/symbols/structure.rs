use crate::ids::{SirClassId, SirEnumId, SirFieldId, SirFnId, SirTraitId, SirTypeId, SirValueId};

#[derive(Debug, Clone)]
pub struct SirValueSymbol {
    pub id: SirValueId,
    pub name: String,
    pub ty: Option<SirTypeId>,
}

#[derive(Debug, Clone)]
pub struct SirFunctionSymbol {
    pub id: SirFnId,
    pub name: String,
    pub params: Vec<SirValueId>,
    pub ret: Option<SirTypeId>,
}

#[derive(Debug, Clone)]
pub struct SirClassSymbol {
    pub id: SirClassId,
    pub name: String,
    pub fields: Vec<SirFieldId>,
    pub methods: Vec<SirFnId>,
    pub inherited_class: Option<SirClassId>,
    pub implemented_traits: Vec<SirTraitId>,
    pub delegated_trait_impls: Vec<SirTraitImpl>,
}

#[derive(Debug, Clone)]
pub struct SirTraitSymbol {
    pub id: SirTraitId,
    pub name: String,
    pub methods: Vec<SirFnId>,
}

#[derive(Debug, Clone)]
pub struct SirFieldSymbol {
    pub id: SirFieldId,
    pub owner: SirClassId,
    pub name: String,
    pub ty: Option<SirTypeId>,
}

#[derive(Debug, Clone)]
pub struct SirEnumSymbol {
    pub id: SirEnumId,
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SirTraitImpl {
    pub trait_id: SirTraitId,
    pub via_member: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SirLinkTarget {
    Value(SirValueId),
    Function(SirFnId),
    Class(SirClassId),
    Enum(SirEnumId),
    Trait(SirTraitId),
    Type(SirTypeId),
}

#[derive(Debug, Clone)]
pub struct SirImportLink {
    pub local_name: String,
    pub from_module_path: String,
    pub target: Option<SirLinkTarget>,
}

#[derive(Debug, Clone)]
pub struct SirExportLink {
    pub name: String,
    pub target: Option<SirLinkTarget>,
}

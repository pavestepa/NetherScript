mod class_statmnt;
pub use class_statmnt::ClassStatmnt;
mod enum_statmnt;
pub use enum_statmnt::EnumStatmnt;
mod fn_statmnt;
pub use fn_statmnt::{FnStatmnt, FnStatmntExpr};
mod import_statmnt;
pub use import_statmnt::ImportStatmnt;
mod trait_statmnt;
pub use trait_statmnt::TraitStatmnt;

pub enum Statmnt {
    ClassStatmnt(ClassStatmnt),
    EnumStatmnt(EnumStatmnt),
    FnStatmnt(FnStatmnt),
    ImportStatmnt(ImportStatmnt),
    TraitStatmnt(TraitStatmnt),
}

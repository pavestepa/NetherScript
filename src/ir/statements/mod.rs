mod mod_statmnt; pub use mod_statmnt::ModStatmnt;
mod use_statmnt; pub use use_statmnt::UseStatmnt;

mod fn_statmnt; pub use fn_statmnt::FnStatmnt;
mod global_const_statmnt; pub use global_const_statmnt::GlobalConstStatmnt;
mod struct_statmnt; pub use struct_statmnt::StructStatmnt;
mod trait_statmnt; pub use trait_statmnt::TraitStatmnt;
mod enum_statmnt; pub use enum_statmnt::EnumStatmnt;

pub enum Statement {
  FnStatmnt(FnStatmnt),
  GlobalConstStatmnt(GlobalConstStatmnt),
  StructStatmnt(StructStatmnt),
  TraitStatmnt(TraitStatmnt),
  EnumStatmnt(EnumStatmnt),
}
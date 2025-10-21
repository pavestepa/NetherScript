pub mod trait_member;
pub use trait_member::TraitMember;
pub struct TraitStatmnt {
    pub is_pub: bool,
    pub name: String,
    pub members: Vec<TraitMember>,
}

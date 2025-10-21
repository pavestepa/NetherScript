pub mod class_constructor;
pub mod class_field;
pub mod class_method;

pub struct ClassStatmnt {
  is_pub: bool,
  name: String,
  extends: Option<String>,
  implements: Option<Vec<String>>,

}
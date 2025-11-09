pub struct LifetimeNode {
  start: String,
  end: String,
  owners: Vec<String>,
  borrows: Vec<String>,
  escape: bool,
}
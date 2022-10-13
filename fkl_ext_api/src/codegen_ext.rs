pub struct CodegenExt {
  pub name: String,
  pub parser: String,
}

pub trait CodegenTrait {
  fn new(name: &str) -> Self;
  fn parser(&self) -> &str;
}

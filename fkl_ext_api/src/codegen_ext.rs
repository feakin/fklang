pub struct CodegenExt {
  pub name: String,
  pub parser: String,
}

pub trait CodegenTrait {
  fn name(&self) -> String;
  fn parser(&self) -> String;
}

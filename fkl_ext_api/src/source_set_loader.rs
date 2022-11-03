pub trait SourceSetLoaderTrait {
  fn name() -> String;
  fn load(&self, path: &str) -> String;
}

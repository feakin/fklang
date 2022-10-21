pub trait SourceSetLoaderTrait {
  fn new() -> Self;
  fn load(&self, path: &str) -> String;
}

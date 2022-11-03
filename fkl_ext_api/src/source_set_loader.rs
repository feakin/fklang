pub trait SourceSetLoaderTrait {
  fn name() -> String;
  fn load(path: String) -> String;
}

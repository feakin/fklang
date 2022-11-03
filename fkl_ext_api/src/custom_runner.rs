pub trait CustomRunnerTrait {
  fn new() -> Self;
  fn run(&self, context: &str);
}

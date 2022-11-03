use fkl_mir::ContextMap;

/// A custom runner is a function that takes a context map and returns a result.
pub trait CustomRunnerTrait {
  fn new() -> Self;
  fn execute(&self, context: &ContextMap);
}

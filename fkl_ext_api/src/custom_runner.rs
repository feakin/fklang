use fkl_mir::ContextMap;

pub trait CustomRunnerTrait {
  fn new() -> Self;
  fn execute(&self, context: &ContextMap);
}

use std::any::Any;
use fkl_mir::{ContextMap, CustomEnv};
use downcast_rs::{impl_downcast, Downcast};

/// A custom runner is a function that takes a context map and returns a result.
pub trait CustomRunner: Downcast + Any + Send + Sync {
  /// name of the custom runner for debugging purposes
  fn name(&self) -> &str {
    std::any::type_name::<Self>()
  }
  /// run the custom runner
  fn execute(&self, context: &ContextMap, env: &CustomEnv);
}

impl_downcast!(CustomRunner);

pub type CreateRunner = unsafe fn() -> *mut dyn CustomRunner;

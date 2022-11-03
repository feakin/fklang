use std::any::Any;

use async_trait::async_trait;
use downcast_rs::{Downcast, impl_downcast};

use fkl_mir::{ContextMap, CustomEnv};

/// A custom runner is a function that takes a context map and returns a result.
#[async_trait]
pub trait CustomRunner: Downcast + Any + Send + Sync {
  /// name of the custom runner for debugging purposes
  fn name(&self) -> &str {
    std::any::type_name::<Self>()
  }
  /// run the custom runner
  async fn execute(&self, context: &ContextMap, env: &CustomEnv);
}

impl_downcast!(CustomRunner);

pub type CreateRunner = unsafe fn() -> *mut dyn CustomRunner;

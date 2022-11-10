use std::any::Any;

use async_trait::async_trait;
use downcast_rs::{Downcast, impl_downcast};

use fkl_mir::{ContextMap, CustomEnv};

pub struct Argument {
  pub name: String,
  pub value: String,
}

/// A custom runner is a function that takes a context map and returns a result.
#[async_trait]
pub trait CustomRunner: Downcast + Any + Send + Sync {
  /// name of the custom runner for debugging purposes
  fn name(&self) -> &str {
    std::any::type_name::<Self>()
  }
  /// run the custom runner
  async fn execute(&self, context: &ContextMap, env: &CustomEnv);

  /// send command to the custom runner
  async fn send_command(&self, _command: &str, _args: &[Argument]) -> Option<String> {
    None
  }

  /// list custom commands
  fn list_commands(&self) -> Vec<String> {
    Vec::new()
  }
}

impl_downcast!(CustomRunner);

pub type CreateRunner = unsafe fn() -> *mut dyn CustomRunner;

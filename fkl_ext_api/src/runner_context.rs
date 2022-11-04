use std::collections::HashSet;

use fkl_mir::{ContextMap, CustomEnv};

use crate::custom_runner::CustomRunner;

pub struct RunnerContext {
  pub context: ContextMap,
  pub env: CustomEnv,

  /// some plugins may need to store data in the context
  plugins: Vec<Box<dyn CustomRunner>>,
  plugin_names: HashSet<String>,
}

impl RunnerContext {
  pub fn new(context: ContextMap, env: CustomEnv) -> Self {
    Self {
      context,
      env,
      plugins: Vec::new(),
      plugin_names: HashSet::new(),
    }
  }

  pub fn add_plugin(&mut self, plugin: Box<dyn CustomRunner>) {
    if self.plugin_names.contains(plugin.name()) {
      panic!("plugin {} already exists", plugin.name());
    }
    self.plugins.push(plugin);
    self.plugin_names.insert(plugin.name().to_string());
  }

  pub async fn execute_plugins(&self) {
    for plugin in &self.plugins {
      plugin.execute(&self.context, &self.env).await;
    }
  }
}


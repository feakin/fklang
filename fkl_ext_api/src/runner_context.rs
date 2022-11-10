use std::collections::HashSet;

use fkl_mir::{ContextMap, CustomEnv};

use crate::custom_runner::CustomRunner;

/// A runner context is a map of custom runners which can supported for DAG style called.
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
    let plugin_name: String = plugin.name().to_string();
    let name = &plugin_name;

    if self.plugin_names.contains(name) {
      panic!("plugin {} already exists", name);
    }
    self.plugins.push(plugin);
    self.plugin_names.insert(plugin_name);
  }

  pub async fn execute_plugins(&self) {
    for plugin in &self.plugins {
      plugin.execute(&self.context, &self.env).await;
    }
  }
}


use std::collections::HashMap;
use fkl_mir::LayeredArchitecture;

use crate::code_meta::CodeFile;

#[derive(Debug, Clone)]
pub struct PackageGuarding {
  pub all_layer: Vec<String>,
  pub rules: Vec<PackageRule>,
}

#[derive(Debug, Clone)]
pub struct PackageRule {
  pub source: String,
  pub targets: Vec<String>,
}

impl PackageGuarding {
  pub fn new() -> Self {
    PackageGuarding {
      all_layer: vec![],
      rules: vec![],
    }
  }

  pub fn add_rule(&mut self, rule: PackageRule) {
    self.rules.push(rule);
  }

  pub fn from_arch(arch: &LayeredArchitecture) -> Self {
    let mut guarding = PackageGuarding::new();
    let mut layered_relations: HashMap<String, Vec<String>> = HashMap::new();
    let mut layered_name_map: HashMap<String, String> = HashMap::new();

    let _ = &arch.layers.iter().for_each(|layer| {
      let layer_name = layer.name.clone();
      layered_name_map.insert(layer_name.clone(), layer.package.clone());
    });

    for relation in &arch.relations {
      let source_name = layered_name_map.get(&relation.source).unwrap();
      let target_name = layered_name_map.get(&relation.target).unwrap();

      layered_relations
        .entry(source_name.clone())
        .or_insert(vec![])
        .push(target_name.clone());
    }

    guarding.all_layer = layered_name_map.values().map(|v| v.clone()).collect();

    // insert rules for empty layer
    guarding.all_layer.iter().for_each(|layer| {
      if let None = layered_relations.get(layer) {
        layered_relations.insert(layer.clone(), vec![]);
      }
    });

    for (source, targets) in layered_relations {
      guarding.add_rule(PackageRule {
        source,
        targets,
      });
    }

    guarding
  }

  pub fn verify(&self, file: &CodeFile) -> Vec<String> {
    let mut errors: Vec<String> = vec![];

    for rule in &self.rules {
      if file.package.starts_with(&rule.source) {
        self.filter_with_imports(&file, &mut errors, &rule);
      }
    }

    return errors;
  }

  fn filter_with_imports(&self, file: &&CodeFile, errors: &mut Vec<String>, rule: &&PackageRule) {
    for import in &file.imports {
      let package_name = package_name(import).to_string();
      if self.all_layer.contains(&package_name) && !rule.targets.contains(&package_name) {
        errors.push(format!("package {} imported {}", file.package, package_name));
      }
    }
  }
}

pub fn package_name(package_name: &str) -> &str {
  let mut pkg = package_name;
  if let Some(index) = pkg.rfind(".") {
    pkg = &pkg[0..index];
  }

  pkg
}


#[cfg(test)]
mod tests {
  use crate::construct::code_construct::CodeConstruct;
  use crate::construct::java_construct::JavaConstruct;

  use crate::builtin::funcs::{mir_from_str};
  use crate::builtin::funcs::layered_guarding::package_guarding::{PackageGuarding, package_name};

  #[test]
  fn pure_package_name() {
    assert_eq!(package_name("com.intellij.psi.StubBuilder"), "com.intellij.psi");
    assert_eq!(package_name("com.intellij.psi.*"), "com.intellij.psi");
  }

  fn sample_layer() -> &'static str {
    r#"
layered DDD {
  dependency {
    interface -> application
    interface -> domain
    interface -> infrastructure
    application -> domain
    application -> infrastructure
  }
  layer interface {
    package: "com.phodal.rest";
  }
  layer domain {
    package: "com.phodal.domain";
  }
  layer application {
    package: "com.phodal.application";
  }
  layer infrastructure {
    package: "com.phodal.infrastructure";
  }
}
"#
  }

  #[test]
  fn guarding_package_for_error() {
    let java_code = r#"
package com.phodal.domain;

import com.phodal.application.UserApplicationService;

class Demo {}
"#;


    let file = JavaConstruct::parse(java_code);
    let context = mir_from_str(sample_layer());

    let arch = context.layered.unwrap();
    let guarding = PackageGuarding::from_arch(&arch);
    let errors = guarding.verify(&file);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0], "package com.phodal.domain imported com.phodal.application");
  }

  #[test]
  fn guarding_package_for_normal() {
    let java_code = r#"
package com.phodal.application;

import com.phodal.domain;
import com.phodal.infrastucture;
import java.util.Scanner;

class Demo {}
"#;


    let file = JavaConstruct::parse(java_code);
    let context = mir_from_str(sample_layer());

    let arch = context.layered.unwrap();
    let guarding = PackageGuarding::from_arch(&arch);
    let errors = guarding.verify(&file);

    assert_eq!(errors.len(), 0);
  }
}

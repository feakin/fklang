use std::path::PathBuf;

use crate::builtin::funcs::LayerMap;

pub struct LayerPathBuilder {}

impl LayerPathBuilder {
  pub fn domain(base: &PathBuf, layer: Option<&LayerMap>, class_name: &str) -> PathBuf {
    let mut target_path = base.clone();
    let package_path = layer
      .map(|layer| layer.domain_path())
      .filter(|path| path != "src/main/java/")
      .unwrap_or_else(|| "src/main/java/domain".to_string());
    target_path.push(package_path);
    target_path.push(format!("{}.java", class_name));
    target_path
  }

  pub fn controller(base: &PathBuf, layer: &LayerMap, class_name: String) -> String {
    let file_name = class_name + "Controller.java";
    let mut target_path = base.clone();
    target_path.push(layer.interface_path().clone());
    target_path.push(file_name);

    // todo: create target file
    if !target_path.exists() {
      panic!("target file not found: {}", target_path.to_str().unwrap());
    }

    format!("{}", target_path.display())
  }
}

use std::path::PathBuf;

use crate::exec::layer_map::LayerMap;

pub struct LayerPathBuilder {}

impl LayerPathBuilder {
  pub fn controller(base: &PathBuf, layer: &LayerMap, class_name: String) -> String {
    let file_name = class_name + "Controller.java";
    let mut target_path = base.clone();
    target_path.push(layer.interface_path().clone());
    target_path.push(file_name);

    if !target_path.exists() {
      panic!("target file not found: {}", target_path.to_str().unwrap());
    }

    format!("{}", target_path.display())
  }
}

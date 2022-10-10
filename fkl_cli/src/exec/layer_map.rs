use log::error;
use fkl_parser::mir::LayeredArchitecture;

pub struct LayerMap {
  pub interface: String,
  pub application: String,
  pub domain: String,
  pub infrastructure: String,
}

impl Default for LayerMap {
  fn default() -> Self {
    LayerMap {
      interface: "".to_string(),
      application: "".to_string(),
      domain: "".to_string(),
      infrastructure: "".to_string(),
    }
  }
}

impl LayerMap {
  pub fn from(layered: LayeredArchitecture) -> LayerMap {
    let mut map = LayerMap {
      interface: "".to_string(),
      application: "".to_string(),
      domain: "".to_string(),
      infrastructure: "".to_string(),
    };

    for layer in &layered.layers {
      let string: &str = &layer.name;
      match string {
        "interface" => map.interface = layer.package.clone(),
        "application" => map.application = layer.package.clone(),
        "domain" => map.domain = layer.package.clone(),
        "infrastructure" => map.infrastructure = layer.package.clone(),
        _ => error!("Unknown layer name: {}", layer.name),
      }
    }

    map
  }

  pub fn interface_path(&self) -> String {
    java_package_to_path(&self.interface)
  }

  pub fn application_path(&self) -> String {
    java_package_to_path(&self.application)
  }

  pub fn domain_path(&self) -> String {
    java_package_to_path(&self.domain)
  }

  pub fn infrastructure_path(&self) -> String {
    java_package_to_path(&self.infrastructure)
  }
}

// "com.feakin.fklang" => "src/main/java/com/feakin.fklang"
pub fn java_package_to_path(package: &str) -> String {
  let mut path = String::from("src/main/java/");
  path.push_str(&*package.replace(".", "/"));
  path
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::{Layer, LayeredArchitecture};
  use crate::exec::layer_map::{java_package_to_path, LayerMap};

  #[test]
  fn package_convert() {
    assert_eq!(java_package_to_path("com.feakin.fklang"), "src/main/java/com/feakin/fklang")
  }

  #[test]
  fn package_convert_with_trailing_slash() {
    let layer_map = LayerMap::from(LayeredArchitecture {
      name: "".to_string(),
      description: "".to_string(),
      dependencies: vec![],
      layers: vec![
        Layer {
          name: "interface".to_string(),
          package: "com.feakin.fklang".to_string(),
        },
      ]
    });

    assert_eq!(layer_map.interface_path(), "src/main/java/com/feakin/fklang")
  }
}


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
      infrastructure: "".to_string()
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

  pub fn interface_path(&self) {

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
  use crate::exec::layer_map::java_package_to_path;

  #[test]
  fn package_convert() {
    assert_eq!(java_package_to_path("com.feakin.fklang"), "src/main/java/com/feakin/fklang")
  }
}


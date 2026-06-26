use std::fs;
use std::path::{Path, PathBuf};

use libloading::{Library, Symbol};
use serde::Deserialize;
use thiserror::Error;
use fkl_ext_api::custom_runner::{CreateRunner, CustomRunner};

/// Errors that can occur when loading a dynamic ext
#[derive(Debug, Error)]
pub enum ExtLoadError {
  #[error("cannot load library: {0}")]
  Library(libloading::Error),
  #[error("dynamic library does not contain a valid dynamic ext")]
  Plugin(libloading::Error),
  #[error("cannot read plugin registry: {0}")]
  RegistryIo(#[from] std::io::Error),
  #[error("cannot parse plugin manifest: {0}")]
  RegistryParse(#[from] toml::de::Error),
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PluginManifest {
  pub name: String,
  pub kind: PluginKind,
  pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PluginKind {
  CustomRunner,
  DatasourceConnector,
  Codegen,
}

pub fn load_registry(registry_dir: impl AsRef<Path>) -> Result<Vec<PluginManifest>, ExtLoadError> {
  let registry_dir = registry_dir.as_ref();
  let mut plugins = Vec::new();

  for entry in fs::read_dir(registry_dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
      continue;
    }

    let mut manifest: PluginManifest = toml::from_str(&fs::read_to_string(&path)?)?;
    if manifest.path.is_relative() {
      manifest.path = registry_dir.join(&manifest.path);
    }
    plugins.push(manifest);
  }

  plugins.sort_by(|left, right| left.name.cmp(&right.name));
  Ok(plugins)
}

/// links a ext at the given path.
pub unsafe fn dynamically_load_ext(
  path: &str,
) -> Result<(Library, Box<dyn CustomRunner>), ExtLoadError> {
  // 1. load the dynamic library
  let lib = Library::new(path).map_err(ExtLoadError::Library)?;

  // 2. get and check the function pointer
  let func: Symbol<CreateRunner> = lib
    .get(b"_fkl_create_runner")
    .map_err(ExtLoadError::Plugin)?;

  // 3. call the function pointer
  let plugin = Box::from_raw(func());
  Ok((lib, plugin))
}

#[cfg(target_os = "macos")]
pub fn ext_path(plugin_name: &str, for_production: bool) -> String {
  if for_production {
    format!("plugins/lib{}.dylib", plugin_name)
  } else {
    format!("target/debug/lib{}.dylib", plugin_name)
  }
}

#[cfg(target_os = "linux")]
pub fn ext_path(plugin_name: &str, for_production: bool) -> String {
  if for_production {
    format!("plugins/lib{}.so", plugin_name)
  } else {
    format!("target/debug/lib{}.so", plugin_name)
  }
}

#[cfg(target_os = "windows")]
pub fn ext_path(plugin_name: &str, for_production: bool) -> String {
  if for_production {
    format!("plugins\\{}.dll", plugin_name)
  } else {
    format!("target\\debug\\{}.dll", plugin_name)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;
  use std::time::{SystemTime, UNIX_EPOCH};
  use fkl_mir::{ContextMap, CustomEnv};

  #[test]
  fn loads_plugin_manifests_from_registry_directory() {
    let registry_dir = std::env::temp_dir().join(format!(
      "fkl-plugin-registry-{}-{}",
      std::process::id(),
      SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir_all(&registry_dir).unwrap();
    std::fs::write(
      registry_dir.join("hello.toml"),
      r#"name = "hello"
kind = "custom-runner"
path = "plugins/libhello.dylib"
"#,
    ).unwrap();

    let plugins = load_registry(&registry_dir).unwrap();

    std::fs::remove_dir_all(&registry_dir).unwrap();
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0].name, "hello");
    assert_eq!(plugins[0].kind, PluginKind::CustomRunner);
    assert_eq!(plugins[0].path, registry_dir.join("plugins/libhello.dylib"));
  }

  #[tokio::test]
  #[ignore = "requires target/debug/libext_hello_world dynamic library"]
  async fn test_load_ext() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .parent().unwrap()
      .join(ext_path("ext_hello_world", false));

    unsafe {
      let (lib, ext) = dynamically_load_ext(path.to_str().unwrap()).unwrap();
      std::mem::forget(lib); // Ensure that the library is not automatically unloaded
      // println!("ext: {:?}", ext);
      ext.execute(&ContextMap::default(), &CustomEnv::default()).await;
    }
  }
}

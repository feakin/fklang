use libloading::{Library, Symbol};
use thiserror::Error;
use fkl_ext_api::custom_runner::{CreateRunner, CustomRunner};

/// Errors that can occur when loading a dynamic ext
#[derive(Debug, Error)]
pub enum ExtLoadError {
  #[error("cannot load library: {0}")]
  Library(libloading::Error),
  #[error("dynamic library does not contain a valid dynamic ext")]
  Plugin(libloading::Error),
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
  use fkl_mir::{ContextMap, CustomEnv};

  #[tokio::test]
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

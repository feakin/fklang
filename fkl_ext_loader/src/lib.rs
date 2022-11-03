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
  let lib = Library::new(path).map_err(ExtLoadError::Library)?;
  let func: Symbol<CreateRunner> = lib
    .get(b"_fkl_create_runner")
    .map_err(ExtLoadError::Plugin)?;
  let plugin = Box::from_raw(func());
  Ok((lib, plugin))
}

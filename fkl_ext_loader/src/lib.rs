use thiserror::Error;

/// Errors that can occur when loading a dynamic plugin
#[derive(Debug, Error)]
pub enum ExtLoadError {
  #[error("cannot load library: {0}")]
  Library(libloading::Error),
  #[error("dynamic library does not contain a valid dynamic ext")]
  Plugin(libloading::Error),
}

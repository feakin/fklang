use crate::code_meta::CodeFile;

#[derive(Debug, Clone)]
pub struct LayeredGuardingExec {
  pub directions: Vec<LayerDirection>,
  pub models: Vec<CodeFile>,
}

#[derive(Debug, Clone)]
pub struct LayerDirection {
  pub source: String,
  pub target: String,
}

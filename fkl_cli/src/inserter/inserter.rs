use crate::code_meta::CodeClass;

pub trait Inserter {
  fn insert(path: &str, clazz: &CodeClass, lines: &Vec<String>) -> Result<(), String>;
}

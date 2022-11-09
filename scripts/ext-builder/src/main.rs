use std::env;
use std::path::PathBuf;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("args: {:?}", args);
  let is_production = args.len() > 1 && args[1] == "production";

  //   create dir if not exists
  let mut target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .parent().unwrap()
    .parent().unwrap()
    .to_path_buf();

  let mut source_dir = target_dir.clone();

  if is_production {
    source_dir.push("target/release");
  } else {
    source_dir.push("target/debug");
  }

  target_dir.push("plugins");
  if !target_dir.exists() {
    std::fs::create_dir(&target_dir).unwrap();
  }

  let plugin_names = vec!["fkl_ext_kafka"];
  for plugin_name in plugin_names {
    copy_plugin_by_os(plugin_name, &source_dir, &target_dir);
  };
}

#[cfg(target_os = "macos")]
fn copy_plugin_by_os(plugin_name: &str, source_dir: &PathBuf, target_dir: &PathBuf) {
  let mut source_path = source_dir.clone();
  source_path.push(format!("lib{}.dylib", plugin_name));

  let mut target_path = target_dir.clone();
  target_path.push(format!("lib{}.dylib", plugin_name));

  println!("copy from {:?} to {:?}", source_path, target_path);
  std::fs::copy(source_path, target_path).unwrap();
}

#[cfg(target_os = "linux")]
fn copy_plugin_by_os(plugin_name: &str, source_dir: &PathBuf, target_dir: &PathBuf) {
  let mut source_path = source_dir.clone();
  source_path.push(format!("lib{}.so", plugin_name));

  let mut target_path = target_dir.clone();
  target_path.push(format!("lib{}.so", plugin_name));

  println!("copy from {:?} to {:?}", source_path, target_path);
  std::fs::copy(source_path, target_path).unwrap();
}

#[cfg(target_os = "windows")]
fn copy_plugin_by_os(plugin_name: &str, source_dir: &PathBuf, target_dir: &PathBuf) {
  let mut source_path = source_dir.clone();
  source_path.push(format!("{}.dll", plugin_name));

  let mut target_path = target_dir.clone();
  target_path.push(format!("{}.dll", plugin_name));

  println!("copy from {:?} to {:?}", source_path, target_path);
  std::fs::copy(source_path, target_path).unwrap();
}

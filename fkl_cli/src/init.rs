use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InitOptions {
  pub name: String,
  pub path: PathBuf,
  pub force: bool,
}

pub fn init_project(options: InitOptions) -> io::Result<PathBuf> {
  fs::create_dir_all(&options.path)?;
  let main = options.path.join("main.fkl");

  if main.exists() && !options.force {
    return Err(io::Error::new(
      io::ErrorKind::AlreadyExists,
      format!("{} already exists", main.display()),
    ));
  }

  fs::write(&main, template(&options.name))?;
  Ok(main)
}

fn template(name: &str) -> String {
  format!(
    r#"ContextMap {name} {{
  Context Core {{
    Aggregate Sample;
  }}
}}
"#
  )
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;
  use std::time::{SystemTime, UNIX_EPOCH};

  use super::{init_project, InitOptions};

  #[test]
  fn init_project_creates_parseable_main_fkl_template() {
    let dir = test_dir("creates_template");

    let main = init_project(InitOptions {
      name: "TicketBooking".to_string(),
      path: dir.clone(),
      force: false,
    })
    .expect("init project");

    assert_eq!(main, dir.join("main.fkl"));
    let source = fs::read_to_string(&main).expect("main.fkl");
    assert!(source.contains("ContextMap TicketBooking"));
    fkl_parser::parse(&source).expect("parse generated template");
  }

  #[test]
  fn init_project_refuses_to_overwrite_existing_main_fkl() {
    let dir = test_dir("refuses_overwrite");
    fs::create_dir_all(&dir).expect("create test dir");
    fs::write(dir.join("main.fkl"), "ContextMap Existing {}").expect("seed main.fkl");

    let error = init_project(InitOptions {
      name: "TicketBooking".to_string(),
      path: dir.clone(),
      force: false,
    })
    .expect_err("existing main.fkl should fail");

    assert_eq!(error.kind(), std::io::ErrorKind::AlreadyExists);
    assert_eq!(
      fs::read_to_string(dir.join("main.fkl")).expect("main.fkl"),
      "ContextMap Existing {}"
    );
  }

  fn test_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("system time")
      .as_nanos();
    std::env::temp_dir().join(format!("fkl_init_{name}_{nanos}"))
  }
}

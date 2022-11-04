use std::sync::{Arc, Mutex};

use salsa::DebugWithDb;

fn calc() {
  let mut db = Database::default();
}

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub(crate) struct Database {
  storage: salsa::Storage<Self>,
  // The logs are only used for testing and demonstrating reuse:
  logs: Option<Arc<Mutex<Vec<String>>>>,
}

impl salsa::Database for Database {
  fn salsa_event(&self, event: salsa::Event) {
    // Log interesting events, if logging is enabled
    if let Some(logs) = &self.logs {
      // don't log boring events
      if let salsa::EventKind::WillExecute { .. } = event.kind {
        logs.lock()
          .unwrap()
          .push(format!("Event: {:?}", event.debug(self)));
      }
    }
  }
}

#[salsa::tracked]
pub(crate) struct File {
  contents: String,
}

#[salsa::input]
pub struct SourceProgram {
  pub text: String,
}

#[salsa::tracked]
pub fn sum(db: &dyn crate::Db) -> u32 {
  return 0;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    calc();
  }
}

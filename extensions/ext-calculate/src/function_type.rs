use std::collections::HashMap;

/// main type: immutable, and mutable
/// special case: async
pub enum FunctionType {
  /// A function that is not tracked.
  Untracked,
  /// A function that is tracked.
  Tracked,
  /// A function that is tracked and has a `&mut` parameter.
  TrackedMut,
  /// A function that is Async
  Async,
}

pub struct FuncHub {
  pub functions: HashMap<String, Box<dyn Fn()>>,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut hub = FuncHub {
      functions: HashMap::new(),
    };
  
    hub.functions.insert("test".to_string(), Box::new(|| println!("test")));
    let test = hub.functions.get("test").unwrap();
    test();
  }
}

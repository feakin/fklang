pub fn naming(name: &str) -> String {
  let mut new_name = name.to_string();
  if !name.starts_with("cluster_") {
    new_name = format!("cluster_{}", name);
  }

  to_snakecase(&new_name)
}

pub(crate) fn to_snakecase(name: &str) -> String {
  let chars = name.chars();
  let mut result = String::new();
  for c in chars {
    if c == ' ' {
      result.push('_');
      continue;
    } else if c.is_uppercase() {
      result.push(c.to_ascii_lowercase());
    } else {
      result.push(c);
    }
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_to_snakecase() {
    assert_eq!(to_snakecase("Hello World"), "hello_world");
    assert_eq!(to_snakecase("HelloWorld"), "helloworld");
    assert_eq!(to_snakecase("hello_world"), "hello_world");
    assert_eq!(to_snakecase("helloWorld"), "helloworld");
  }

  #[test]
  fn return_origin_when_correct() {
    assert_eq!(naming("cluster_hello_world"), "cluster_hello_world");
    assert_eq!(naming("cluster_helloworld"), "cluster_helloworld");
  }
}

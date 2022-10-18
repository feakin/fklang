#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;
  use crate::code_gen_exec;

  #[test]
  fn test_java_package_to_path() {
    let mut d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/spring");

    let base_path = d.clone();

    let mut input_path = d.clone();
    input_path.push(format!("spring.fkl"));

    code_gen_exec::code_gen_by_path(&input_path, Some(&"HelloGot".to_string()), &base_path);

    let controller = "test_data/spring/src/main/java/com/feakin/demo/rest/HelloController.java";
    let output = fs::read_to_string(controller).expect("Something went wrong reading the file");
    assert_eq!(output, r#"package com.feakin.demo.rest;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class HelloController {

	@GetMapping("/")
	public String index() {
		return "Greetings from Spring Boot!";
	}


    @GetMapping("/hello")
    public String gotHello() {

    }

}"#);

    reset_test(controller);
  }

  #[test]
  #[should_panic]
  fn panic_for_duplicated_method() {
    let mut d: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/spring");

    let base_path = d.clone();

    let mut input_path = d.clone();
    input_path.push(format!("spring.fkl"));

    code_gen_exec::code_gen_by_path(&input_path, Some(&"index".to_string()), &base_path);
  }

  fn reset_test(controller: &str) {
// reset test
    fs::write(controller, r#"package com.feakin.demo.rest;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class HelloController {

	@GetMapping("/")
	public String index() {
		return "Greetings from Spring Boot!";
	}

}
"#).unwrap();
  }
}

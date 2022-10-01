use fkl_parser::mir::implementation::HttpEndpoint;

pub struct SpringCodeGen {
  pub annotation: String,
  pub method_head: String,
  pub ai_comment: String,
}

impl SpringCodeGen {
  pub fn from(http: HttpEndpoint) -> Self {
    let annotation_key = match http.method.to_lowercase().as_str() {
      "get" => "@GetMapping",
      "post" => "@PostMapping",
      "put" => "@PutMapping",
      "delete" => "@DeleteMapping",
      "patch" => "@PatchMapping",
      _ => "@GetMapping",
    };


    let annotation_value: String = match http.path.as_str() {
      "" => "".to_string(),
      _ => "(\"".to_owned() + &http.path.to_owned() + &"\")".to_owned(),
    };

    SpringCodeGen {
      annotation: annotation_key.to_owned() + &*annotation_value,
      method_head: "".to_string(),
      ai_comment: "".to_string()
    }
  }
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::implementation::HttpEndpoint;

  use crate::java_gen::spring_code_gen::SpringCodeGen;

  #[test]
  fn basic_mir() {
    let _output = r#"
import org.springframework.web.bind.annotation.RestController;

  @GetMapping("/employees")
  List<Employee> all() {
    // ... for GitHub Copilot
    return repository.findAll();
  }
"#;
  }

  #[test]
  fn annotation() {
    let annotation = SpringCodeGen::from(HttpEndpoint::default());
    assert_eq!(annotation.annotation, "@GetMapping");

    let annotation = SpringCodeGen::from(HttpEndpoint {
      method: "POST".to_string(),
      ..Default::default()
    });
    assert_eq!(annotation.annotation, "@PostMapping");

    let annotation = SpringCodeGen::from(HttpEndpoint {
      method: "PUT".to_string(),
      path: "/employees".to_string(),
      ..Default::default()
    });

    assert_eq!(annotation.annotation, "@PutMapping(\"/employees\")");
  }
}

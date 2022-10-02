use fkl_parser::mir::implementation::{HttpEndpoint, Response};

/// generate spring code for a single endpoint
/// contains the following:
/// - method annotation
/// - method header
///   - method name
///   - method signature
pub struct SpringCodeGen {
  pub method_annotation: String,
  pub method_header: String,
  pub ai_comment: String,
}

impl SpringCodeGen {
  pub fn from(http: HttpEndpoint) -> Self {
    let method_annotation = Self::method_annotation(&http);
    let method_header = Self::method_header(&http);

    SpringCodeGen {
      method_annotation,
      method_header,
      ai_comment: "".to_string(),
    }
  }

  fn method_annotation(http: &HttpEndpoint) -> String {
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

    let method_annotation = annotation_key.to_owned() + &*annotation_value;
    method_annotation
  }

  fn method_header(http: &HttpEndpoint) -> String {
    let method_name = if http.name == "" {
      "main".to_string()
    } else {
      http.name.to_string()
    };

    let return_type = Self::response_to_return_type(&http.response);
    format!("public {} {}()", return_type, method_name)
  }

  fn response_to_return_type(response: &Option<Response>) -> String {
    match response {
      Some(r) => r.name.to_owned(),
      None => "void".to_owned(),
    }
  }

  fn rename_domain_event_name(str: String) -> String {
    return str;
  }
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::implementation::{HttpEndpoint, Response};

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
    assert_eq!(annotation.method_annotation, "@GetMapping");

    let annotation = SpringCodeGen::from(HttpEndpoint {
      method: "POST".to_string(),
      ..Default::default()
    });
    assert_eq!(annotation.method_annotation, "@PostMapping");

    let annotation = SpringCodeGen::from(HttpEndpoint {
      method: "PUT".to_string(),
      path: "/employees".to_string(),
      ..Default::default()
    });

    assert_eq!(annotation.method_annotation, "@PutMapping(\"/employees\")");
  }

  #[test]
  fn method_head() {
    let annotation = SpringCodeGen::from(HttpEndpoint::default());
    assert_eq!(annotation.method_header, "public void main()");

    let annotation = SpringCodeGen::from(HttpEndpoint {
      name: "all".to_string(),
      description: "".to_string(),
      path: "".to_string(),
      method: "GET".to_string(),
      request: None,
      response: Some(Response {
        name: "List<Employee>".to_string(),
        post_validate: None,
      }),
    });

    assert_eq!(annotation.method_header, "public List<Employee> all()");
  }
}

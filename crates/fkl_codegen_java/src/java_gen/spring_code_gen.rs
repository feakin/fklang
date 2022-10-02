use fkl_parser::mir::implementation::{HttpEndpoint, Request, Response};

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
    let method_name = Self::method_name(http);
    let request = Self::request_to_string(&http.request);
    let return_type = Self::response_to_return_type(&http.response);
    format!("public {} {}({})", return_type, method_name, request)
  }

  fn method_name(http: &HttpEndpoint) -> String {
    let method_name = if http.name == "" {
      "main".to_string()
    } else {
      Self::rename_domain_event_name(&http.name)
    };
    method_name
  }

  fn request_to_string(request: &Option<Request>) -> String {
    match request {
      None => {
        "".to_string()
      }
      Some(req) => {
        let request_name = req.name.to_string();
        format!("@RequestBody {} {}", request_name, "request")
      }
    }
  }

  fn response_to_return_type(response: &Option<Response>) -> String {
    match response {
      Some(r) => r.name.to_owned(),
      None => "void".to_owned(),
    }
  }

  fn rename_domain_event_name(str: &String) -> String {
    // TODO: to ends with
    if Self::start_uppercase(str) && Self::check_past_tense(str) {
      Self::execute_rename(str)
    } else {
      str.to_owned()
    }
  }

  fn check_past_tense(str: &String) -> bool {
    str.ends_with("ed")
  }

  fn start_uppercase(str: &String) -> bool {
    str.chars().next().unwrap().is_uppercase()
  }

  fn execute_rename(str: &String) -> String {
    let words = Self::split_words_by_uppercase(str);
    let mut words = words;
    let last_word = words.pop().unwrap();
    let string = Self::past_tense_to_present_tense(last_word.to_lowercase());
    words.insert(0, string);
    // join words
    words.join("")
  }

  fn split_words_by_uppercase(str: &String) -> Vec<String> {
    let mut words: Vec<String> = vec![];
    let mut word = "".to_string();
    for c in str.chars() {
      if c.is_uppercase() {
        words.push(word);
        word = "".to_string();
      }
      word.push(c);
    };
    words.push(word);

    words
  }

  /// convert past tense to present tense
  /// e.g. Created -> Create
  /// e.p. Deleted -> Delete
  /// todo: use https://github.com/CurrySoftware/rust-stemmers
  fn past_tense_to_present_tense(str: String) -> String {
    let mut chars = str.chars();
    let mut new_str = "".to_string();
    while let Some(c) = chars.next() {
      if c == 'e' {
        if let Some(next) = chars.next() {
          if next == 'd' {
            break;
          } else {
            new_str.push(c);
            new_str.push(next);
          }
        }
      } else {
        new_str.push(c);
      }
    }
    new_str
  }
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::implementation::{HttpEndpoint, Request, Response};

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
  fn method_header_with_response() {
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

  #[test]
  fn method_header_with_request() {
    let annotation = SpringCodeGen::from(HttpEndpoint {
      name: "EmployeeCreated".to_string(),
      description: "".to_string(),
      path: "".to_string(),
      method: "PUT".to_string(),
      request: Some(Request {
        name: "CreateEmployeeRequest".to_string(),
        pre_validate: None,
      }),
      response: None,
    });

    assert_eq!(annotation.method_header, "public void creatEmployee(@RequestBody CreateEmployeeRequest request)");
  }
}

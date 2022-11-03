use fkl_mir::{Flow, HttpMethod};
use fkl_mir::implementation::{HttpEndpoint, Request, Response};

use crate::{comments, naming};

/// generate spring code for a single endpoint
/// contains the following:
/// - method annotation
/// - method header
///   - method name
///   - method signature
pub struct SpringCodeGen {
  pub imports: Vec<String>,
  pub method_annotation: String,
  pub method_header: String,
  pub method_name: String,
  pub ai_comments: Vec<String>,
}

// todo: add api for generate service
impl SpringCodeGen {
  pub fn from(http: &HttpEndpoint, flow: &Option<Flow>) -> Self {
    let method_name = Self::method_name(http);
    let method_annotation = Self::method_annotation(&http);
    let method_header = Self::method_header(&http, &method_name);

    let ai_comments = if let Some(flow) = flow {
      comments::ai_comments(&flow.steps)
    } else {
      vec![]
    };

    SpringCodeGen {
      imports: vec![],
      method_annotation,
      method_header,
      method_name,
      ai_comments,
    }
  }

  fn method_annotation(http: &HttpEndpoint) -> String {
    let annotation_key = match http.method {
      HttpMethod::GET => "@GetMapping",
      HttpMethod::POST => "@PostMapping",
      HttpMethod::PUT => "@PutMapping",
      HttpMethod::DELETE => "@DeleteMapping",
      HttpMethod::PATCH => "@PatchMapping",
      _ => "@GetMapping"
    };


    let annotation_value: String = match http.path.as_str() {
      "" => "".to_string(),
      _ => "(\"".to_owned() + &http.path.to_owned() + &"\")".to_owned(),
    };

    let method_annotation = annotation_key.to_owned() + &*annotation_value;
    method_annotation
  }

  fn method_header(http: &HttpEndpoint, method_name: &String) -> String {
    let request = Self::request_to_string(&http.request);
    let return_type = Self::response_to_return_type(&http.response);
    format!("public {} {}({})", return_type, method_name, request)
  }

  fn method_name(http: &HttpEndpoint) -> String {
    let method_name = if http.name == "" {
      "main".to_string()
    } else {
      naming::from_event(&http.name)
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
}

#[cfg(test)]
mod tests {
  use fkl_mir::HttpMethod;
  use fkl_mir::implementation::{HttpEndpoint, Request, Response};

  use crate::spring_gen::spring_code_gen::SpringCodeGen;

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
    let annotation = SpringCodeGen::from(&HttpEndpoint::default(), &None);
    assert_eq!(annotation.method_annotation, "@GetMapping");

    let annotation = SpringCodeGen::from(&HttpEndpoint {
      method: HttpMethod::POST,
      ..Default::default()
    }, &None,
    );
    assert_eq!(annotation.method_annotation, "@PostMapping");

    let annotation = SpringCodeGen::from(&HttpEndpoint {
      method: HttpMethod::PUT,
      path: "/employees".to_string(),
      ..Default::default()
    }, &None,
    );

    assert_eq!(annotation.method_annotation, "@PutMapping(\"/employees\")");
  }

  #[test]
  fn method_header_with_response() {
    let annotation = SpringCodeGen::from(&HttpEndpoint::default(), &None);
    assert_eq!(annotation.method_header, "public void main()");

    let annotation = SpringCodeGen::from(&HttpEndpoint {
      name: "all".to_string(),
      description: "".to_string(),
      path: "".to_string(),
      auth: None,
      method: HttpMethod::GET,
      request: None,
      response: Some(Response {
        name: "List<Employee>".to_string(),
        post_validate: None,
      }),
    }, &None,
    );

    assert_eq!(annotation.method_header, "public List<Employee> all()");
  }

  #[test]
  fn method_header_with_request() {
    let annotation = SpringCodeGen::from(&HttpEndpoint {
      name: "EmployeeCreated".to_string(),
      description: "".to_string(),
      path: "".to_string(),
      auth: None,
      method: HttpMethod::PUT,
      request: Some(Request {
        name: "CreateEmployeeRequest".to_string(),
        pre_validate: None,
      }),
      response: None,
    }, &None,
    );

    assert_eq!(annotation.method_header, "public void createEmployee(@RequestBody CreateEmployeeRequest request)");
  }
}

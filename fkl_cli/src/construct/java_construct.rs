use tree_sitter::{Node, Parser, Query, QueryCursor};

use crate::code_meta::{CodeClass, CodeFile};
use crate::construct::code_construct::CodeConstruct;

const JAVA_QUERY: &'static str = "
(package_declaration
	(scoped_identifier) @package-name)

(import_declaration
	(scoped_identifier) @import-name)

(program
    (class_declaration
	    name: (identifier) @class-name
        interfaces: (super_interfaces (interface_type_list (type_identifier)  @impl-name))?
        body: (class_body (method_declaration
            (modifiers
                (annotation
                  name: (identifier) @annotation.name
                      arguments: (annotation_argument_list)? @annotation.key_values
                )
            )?
            type: (type_identifier) @return-type
            name: (identifier) @function-name
            parameters: (formal_parameters (formal_parameter
              type: (type_identifier) @param-type
                name: (identifier) @param-name
            ))?
          ))?
    )
)";


pub struct JavaConstruct {
  parser: Parser,
  query: Query,
}

impl JavaConstruct {
  pub fn new() -> JavaConstruct {
    let mut parser = Parser::new();
    let language = tree_sitter_java::language();
    parser.set_language(language).unwrap();

    let query = Query::new(language, &JAVA_QUERY)
      .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();

    JavaConstruct {
      parser,
      query,
    }
  }
}

impl CodeConstruct for JavaConstruct {
  fn parse(code: &str) -> CodeFile {
    let mut ident = JavaConstruct::new();
    JavaConstruct::do_parse(&code, &mut ident)
  }
}

impl JavaConstruct {
  fn do_parse(code: &&str, ident: &mut JavaConstruct) -> CodeFile {
    let tree = ident.parser.parse(code, None).unwrap();
    let text_callback = |n: Node| &code[n.byte_range()];
    let mut query_cursor = QueryCursor::new();
    let captures = query_cursor.captures(&ident.query, tree.root_node(), text_callback);

    let mut code_file = CodeFile::default();
    let mut is_last_node = false;

    let mut class = CodeClass::default();

    let capture_names = ident.query.capture_names();
    let mut last_type = "void".to_string();

    for (mat, capture_index) in captures {
      let capture = mat.captures[capture_index];
      let capture_name = &capture_names[capture.index as usize];

      let text = capture.node.utf8_text((&code).as_ref()).unwrap_or("");
      match capture_name.as_str() {
        "package-name" => {
          code_file.package = text.to_string();
        }
        "import-name" => {
          code_file.imports.push(text.to_string());
        }
        "class-name" => {
          if !class.name.is_empty() {
            code_file.classes.push(class.clone());
            class = CodeClass::default();
          }

          class.name = text.to_string();
          class.package = code_file.package.clone();

          let class_node = capture.node.parent().unwrap();
          JavaConstruct::insert_location(&mut class, class_node);
          if !is_last_node {
            is_last_node = true;
          }
        }
        "impl-name" => {
          class.implements.push(text.to_string());
        }
        "function-name" => {
          let mut function = JavaConstruct::insert_function(capture, text);
          function.return_type = last_type.clone();
          class.functions.push(function);
        }
        "return-type" => {
          last_type = text.to_string();
        }
        "parameter" => {}
        &_ => {
          // println!(
          //   "    pattern: {}, capture: {}, row: {}, text: {:?}",
          //   mat.pattern_index,
          //   capture_name,
          //   capture.node.start_position().row,
          //   capture.node.utf8_text((&code).as_ref()).unwrap_or("")
          // );
        }
      }
    }

    if is_last_node {
      code_file.classes.push(class.clone());
    }

    code_file
  }
}


#[cfg(test)]
mod tests {
  use crate::code_meta::{CodeClass, CodeFunction, CodePoint};
  use crate::construct::code_construct::CodeConstruct;
  use crate::construct::java_construct::JavaConstruct;

  #[test]
  fn should_parse_import() {
    let source_code = "import java.lang.System;
import java.io.InputStream;
import payroll.Employee;
";
    let file = JavaConstruct::parse(source_code);
    assert_eq!(3, file.imports.len());
  }

  #[test]
  fn should_parse_impl_java_class() {
    let source_code = "class DateTimeImpl implements DateTime {
    @Override
    public Date getDate() {
        return new Date();
    }
}";
    let file = JavaConstruct::parse(source_code);
    assert_eq!(1, file.classes.len());
    assert_eq!("DateTimeImpl", file.classes[0].name);
  }

  #[test]
  fn should_parse_normal_java_class() {
    let source_code = "class DateTimeImpl {
    public Date getDate() {
        return new Date();
    }
}";
    let file = JavaConstruct::parse(source_code);
    assert_eq!(1, file.classes.len());
    assert_eq!("DateTimeImpl", file.classes[0].name);
  }

  #[test]
  fn should_parse_multiple_java_class() {
    let source_code = "class DateTimeImpl {
}

class DateTimeImpl2 {
}
";
    let file = JavaConstruct::parse(source_code);
    assert_eq!(2, file.classes.len());
    assert_eq!("DateTimeImpl", file.classes[0].name);
    assert_eq!("DateTimeImpl2", file.classes[1].name);
  }

  #[test]
  fn should_support_package_name() {
    let source_code = "package com.phodal.pepper.powermock;
";
    let file = JavaConstruct::parse(source_code);
    assert_eq!("com.phodal.pepper.powermock", file.package);
  }

  #[test]
  fn should_support_inner_class() {
    let source_code = "class OuterClass {
  int x = 10;

  class InnerClass {
    int y = 5;
  }
}";

    let file = JavaConstruct::parse(source_code);
    assert_eq!(1, file.classes.len());
  }

  #[test]
  fn get_location_from_class() {
    let source_code = "class DateTimeImpl {
    public Date getDate() {
        return new Date();
    }
}";

    let file = JavaConstruct::parse(source_code);
    assert_eq!(file.classes[0], CodeClass {
      name: "DateTimeImpl".to_string(),
      path: "".to_string(),
      module: "".to_string(),
      package: "".to_string(),
      implements: vec![],
      functions: vec![CodeFunction {
        name: "getDate".to_string(),
        return_type: "".to_string(),
        variable: vec![],
        start: CodePoint { row: 1, column: 4 },
        end: CodePoint { row: 3, column: 5 },
      }],
      start: CodePoint { row: 0, column: 0 },
      end: CodePoint { row: 4, column: 1 },
    });
  }

  #[test]
  fn hello_world_controller() {
    let source_code = r#"@RestController
public class HelloController {

	@GetMapping("/")
	public String index() {
		return "Greetings from Spring Boot!";
	}

}
"#;

    let file = JavaConstruct::parse(source_code);

    assert_eq!(file.classes.len(), 1);
    assert_eq!(file.classes[0], CodeClass {
      name: "HelloController".to_string(),
      path: "".to_string(),
      module: "".to_string(),
      package: "".to_string(),
      implements: vec![],
      functions: vec![CodeFunction {
        name: "index".to_string(),
        return_type: "String".to_string(),
        variable: vec![],
        start: CodePoint { row: 3, column: 1 },
        end: CodePoint { row: 6, column: 2 },
      }],
      start: CodePoint { row: 0, column: 0 },
      end: CodePoint { row: 8, column: 1 },
    });
  }
}

use tree_sitter::{Node, Parser, Query, QueryCursor};
use crate::class_info::{CodeClass, CodeFile};
use crate::inserter::base_ident::CodeIdent;

const JAVA_QUERY: &'static str = "
(package_declaration
	(scoped_identifier) @package-name)

(import_declaration
	(scoped_identifier) @import-name)

(method_declaration
	(modifiers
    	(annotation
    		name: (identifier) @annotation.name
            arguments: (annotation_argument_list)? @annotation.key_values
    	)
    )
)

(program
    (class_declaration
	    name: (identifier) @class-name
        interfaces: (super_interfaces (interface_type_list (type_identifier)  @impl-name))?
    )
)

";


pub struct JavaIdent {
  parser: Parser,
  query: Query
}

impl JavaIdent {
  pub fn new() -> JavaIdent {
    let mut parser = Parser::new();
    let language = tree_sitter_java::language();
    parser.set_language(language).unwrap();

    let query = Query::new(language, &JAVA_QUERY)
      .map_err(|e| println!("{}", format!("Query compilation failed: {:?}", e))).unwrap();

    JavaIdent {
      parser,
      query
    }
  }
}

impl CodeIdent for JavaIdent {
  fn parse(code: &str) -> CodeFile {
    let mut ident = JavaIdent::new();
    JavaIdent::do_parse(&code, &mut ident)
  }
}

impl JavaIdent {
  fn do_parse(code: &&str, ident: &mut JavaIdent) -> CodeFile {
    let tree = ident.parser.parse(code, None).unwrap();
    let text_callback = |n: Node| &code[n.byte_range()];
    let mut query_cursor = QueryCursor::new();
    let captures = query_cursor.captures(&ident.query, tree.root_node(), text_callback);

    let mut code_file = CodeFile::default();
    let mut class = CodeClass::default();
    let mut is_last_node = false;

    let capture_names = ident.query.capture_names();

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
          JavaIdent::insert_location(&mut class, class_node);
          if !is_last_node {
            is_last_node = true;
          }
        }
        "impl-name" => {
          class.implements.push(text.to_string());
        }
        "parameter" => {}
        &_ => {
          println!(
            "    pattern: {}, capture: {}, row: {}, text: {:?}",
            mat.pattern_index,
            capture_name,
            capture.node.start_position().row,
            capture.node.utf8_text((&code).as_ref()).unwrap_or("")
          );
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
  use crate::inserter::base_ident::CodeIdent;
  use crate::inserter::java_ident::JavaIdent;

  #[test]
  fn should_parse_import() {
    let source_code = "import java.lang.System;
import java.io.InputStream;
import payroll.Employee;
";
    let file = JavaIdent::parse(source_code);
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
    let file = JavaIdent::parse(source_code);
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
    let file = JavaIdent::parse(source_code);
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
    let file = JavaIdent::parse(source_code);
    assert_eq!(2, file.classes.len());
    assert_eq!("DateTimeImpl", file.classes[0].name);
    assert_eq!("DateTimeImpl2", file.classes[1].name);
  }

  #[test]
  fn should_support_package_name() {
    let source_code = "package com.phodal.pepper.powermock;
";
    let file = JavaIdent::parse(source_code);
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

    let file = JavaIdent::parse(source_code);
    assert_eq!(1, file.classes.len());
  }

}

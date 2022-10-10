use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::ArgMatches;

use fkl_parser::parse;

use crate::gen_exec::run_gen;

pub mod ident;
pub mod class_info;
pub mod inserter;
pub mod gen_exec;
pub mod line_separator;

// todo: add code highlight support
fn main() {
  env_logger::init();

  let cmd = clap::Command::new("fkl")
    .bin_name("fkl")
    .subcommand_required(true)
    .subcommand(
      clap::command!("gen")
        .about("Generate code from a fkl file, current support Java")
        .arg(
          clap::arg!(--"path" <PATH>)
            .value_parser(clap::value_parser!(std::path::PathBuf)),
        )
        .arg(clap::arg!(--"impl" <String>))
    )
    .subcommand(
      clap::command!("dot")
        .about("Generate dot file from a fkl file")
        .arg(
          clap::arg!(--"path" <PATH>)
            .value_parser(clap::value_parser!(std::path::PathBuf)),
        ),
    )
    .subcommand(
      clap::command!("parse")
        .about("Parse a fkl file and print the AST")
        .arg(
          clap::arg!(--"path" <PATH>)
            .value_parser(clap::value_parser!(std::path::PathBuf)),
        ),
    );


  let matches: ArgMatches = cmd.get_matches();
  match matches.subcommand() {
    Some(("gen", matches)) => {
      let feakin_path = matches.get_one::<PathBuf>("path");
      let filter_impl = matches.get_one::<String>("impl");

      run_gen(feakin_path, filter_impl);
    }
    Some(("dot", matches)) => {
      let manifest_path = matches.get_one::<PathBuf>("path");
      if let Some(path) = manifest_path {
        gen_to_dot(path);
      } else {
        panic!("Please provide a path to a manifest file");
      }
    }
    Some(("ast", matches)) => {
      let manifest_path = matches.get_one::<PathBuf>("path");
      if let Some(path) = manifest_path {
        parse_to_ast(path);
      } else {
        panic!("Please provide a path to a manifest file");
      }
    }
    _ => unreachable!("clap should ensure we don't get here"),
  };
}

fn gen_to_dot(path: &PathBuf) {
  // read path to string
  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  let context_map = parse(&*contents).expect("TODO: panic message");

  // struct to json
  let json = serde_json::to_string(&context_map).expect("TODO: panic message");
  // json to file
  let mut file = std::fs::File::create("test.json").expect("TODO: panic message");
  file.write_all(json.as_bytes()).expect("TODO: panic message");
}


fn parse_to_ast(path: &PathBuf) {
  // read path to string
  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  let context_map = parse(&*contents).expect("TODO: panic message");

  // struct to json
  let json = serde_json::to_string(&context_map).expect("TODO: panic message");
  // json to file
  let mut file = std::fs::File::create("ast.json").expect("TODO: panic message");
  file.write_all(json.as_bytes()).expect("TODO: panic message");
}

#[cfg(test)]
mod tests {
  use fkl_codegen_java::gen_http_api;
  use fkl_parser::mir::ContextMap;
  use fkl_parser::mir::implementation::Implementation;
  use fkl_parser::parse;

  #[test]
  fn convert_for_cli() {
    let source = r#"impl CinemaCreated {
  endpoint {
    GET "/book/{id}";
    response: Cinema;
  }
}"#;

    let mut output = String::new();
    let context_map: ContextMap = parse(source).unwrap();
    context_map.implementations.iter()
      .for_each(|implementation| {
        match implementation {
          Implementation::PublishHttpApi(http) => {
            output = gen_http_api(http.clone(), "java");
          }
          Implementation::PublishEvent => {}
          Implementation::PublishMessage => {}
        }
      });

    // assert_eq!(output, r#"@GetMapping(\"/book/{id}\")\npublic Cinema creatCinema() { }\n"#)
  }
}

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::ArgMatches;
use log::error;

use fkl_parser::parse;

use crate::exec::{code_gen_exec, mir_from_file};

pub mod construct;
pub mod code_meta;
pub mod inserter;
pub mod exec;
pub mod builtin;
pub mod line_separator;
mod e2e;

// todo: add code highlight support
fn main() {
  env_logger::init_from_env(
    env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

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
      clap::command!("run")
        .about("Run builtin feakin function, like mock, verify or others")
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
  let feakin_path = matches.get_one::<PathBuf>("path");

  if feakin_path.is_none() {
    error!("Please provide a path to a manifest file");
    return;
  }

  let path = feakin_path.unwrap();
  match matches.subcommand() {
    Some(("dot", _matches)) => gen_to_dot(path),
    Some(("ast", _matches)) => parse_to_ast(path),
    Some(("gen", matches)) => {
      let filter_impl = matches.get_one::<String>("impl");
      let parent = path.parent().unwrap().to_path_buf();
      code_gen_exec::code_gen_by_path(path, filter_impl, &parent);
    }
    Some(("run", matches)) => {
      let function = matches.get_one::<String>("func");
      let impl_name = matches.get_one::<String>("impl");

      if function.is_none() {
        error!("Please provide a function name");
        return;
      }

      if impl_name.is_none() {
        error!("Please provide a impl name");
        return;
      }

      let mir = mir_from_file(&feakin_path.unwrap());
      builtin::endpoint_runner::execute(&mir, function.unwrap(), impl_name.unwrap());
    }
    _ => unreachable!("clap should ensure we don't get here"),
  };
}

fn gen_to_dot(path: &PathBuf) {
  // read path to string
  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  let context_map = parse(&*contents).expect("TODO: panic message");

  let json = serde_json::to_string(&context_map).expect("TODO: panic message");

  let mut file = std::fs::File::create("dot.dot").expect("TODO: panic message");
  file.write_all(json.as_bytes()).expect("TODO: panic message");
}


fn parse_to_ast(path: &PathBuf) {
  // read path to string
  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  let context_map = parse(&*contents).expect("TODO: panic message");

  let json = serde_json::to_string(&context_map).expect("TODO: panic message");

  let mut file = fs::File::create("ast.json").expect("TODO: panic message");
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
            output = gen_http_api(&http, "java").code;
          }
          Implementation::PublishEvent => {}
          Implementation::PublishMessage => {}
        }
      });

    // assert_eq!(output, r#"@GetMapping(\"/book/{id}\")\npublic Cinema creatCinema() { }\n"#)
  }
}

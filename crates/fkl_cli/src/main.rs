use std::fs;
use std::io::Write;
use std::path::PathBuf;
use fkl_codegen_java::gen_http_api;
use fkl_parser::mir::implementation::Implementation;

use fkl_parser::parse;

fn main() {
  let cmd = clap::Command::new("fkl")
    .bin_name("fkl")
    .subcommand_required(true)
    .subcommand(
      clap::command!("gen")
        .about("Generate code from a fkl file, current support Java")
        .arg(
          clap::arg!(--"path" <PATH>)
            .value_parser(clap::value_parser!(std::path::PathBuf)),
        ).arg(
        clap::arg!(--"lang" <String>),
      ),
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
    )
    ;


  let matches = cmd.get_matches();
  match matches.subcommand() {
    Some(("gen", matches)) => {
      let feakin_path = matches.get_one::<PathBuf>("path");
      let lang = matches.get_one::<String>("lang");
      // debug
      if let Some(path) = feakin_path {
        if let Some(lang) = lang {
          let feakin = fs::read_to_string(path).unwrap();
          let mir = parse(&feakin).unwrap();

          mir.implementations.iter().for_each(|implementation| {
            match implementation {
              Implementation::PublishHttpApi(http) => {
                let output = gen_http_api(http.clone(), &lang).unwrap();
                println!("{}", output);
              }
              Implementation::PublishEvent => {}
              Implementation::PublishMessage => {}
            }
          });
        }
      }
    }
    Some(("dot", matches)) => {
      let manifest_path = matches.get_one::<PathBuf>("path");
      if let Some(path) = manifest_path {
        gen_to_dot(path);
      } else {
        panic!("Please provide a path to a manifest file");
      }
    }
    Some(("parse", matches)) => {
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

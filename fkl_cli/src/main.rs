use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use fkl_parser::parse;

use crate::exec::{code_gen_exec, mir_from_file};

pub mod construct;
pub mod code_meta;
pub mod inserter;
pub mod exec;
pub mod builtin;
pub mod line_separator;
mod e2e;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  #[command(about = "generate Graphviz/Dot from fkl file")]
  Dot {
    #[arg(short, long)]
    path: PathBuf,
  },
  #[command(about = "generate ast from fkl file")]
  Ast {
    #[arg(short, long)]
    path: PathBuf,
  },
  #[command(about = "generate code from fkl file")]
  Gen {
    #[arg(short, long, required = true)]
    path: PathBuf,
    #[arg(short, long = "impl")]
    impl_name: Option<String>,
  },
  #[command(about = "run function from fkl file")]
  Run(RunOpt),
}

#[derive(Args)]
struct RunOpt {
  #[arg(short, long, required = true)]
  path: PathBuf,
  #[arg(short, required = true, long = "impl")]
  impl_name: String,
  #[arg(short, required = true, long = "func")]
  func_name: RunFuncName,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum RunFuncName {
  HttpRequest,
  // todo: add mock server support
  MockServer,
}

// todo: add code highlight support
fn main() {
  env_logger::init_from_env(
    env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

  let cli: Cli = Cli::parse();
  match &cli.command {
    Commands::Dot { path } => {
      gen_to_dot(path);
    }
    Commands::Ast { path } => {
      parse_to_ast(path);
    }
    Commands::Gen { path, impl_name } => {
      let parent = path.parent().unwrap().to_path_buf();
      code_gen_exec::code_gen_by_path(path, impl_name.clone(), &parent);
    }
    Commands::Run(run) => {
      let mir = mir_from_file(&run.path);
      builtin::endpoint_runner::execute(&mir, &run.func_name, &run.impl_name);
    }
  }
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

  use crate::{builtin, RunFuncName};

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

  #[test]
  #[should_panic]
  fn test_execute_request() {
    let source = r#"impl CinemaCreated {
  endpoint {
    GET "/book/{id}";
    response: Cinema;
  }
}"#;

    let context_map: ContextMap = parse(source).unwrap();

    builtin::endpoint_runner::execute(&context_map, &RunFuncName::HttpRequest, "CinemaCreated");
  }

  #[test]
  #[ignore]
  fn test_normal_request() {
    let source = r#"impl CinemaCreated {
  endpoint {
    GET "https://book.feakin.com/";
    response: Cinema;
  }
}"#;

    let context_map: ContextMap = parse(source).unwrap();

    builtin::endpoint_runner::execute(&context_map, &RunFuncName::HttpRequest, "CinemaCreated");
  }
}

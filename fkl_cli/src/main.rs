use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use log::info;

use fkl_parser::parse;

use crate::exec::{code_gen_exec, LayeredGuardingExec, mir_from_file};

pub mod construct;
pub mod code_meta;
pub mod inserter;
pub mod exec;
pub mod builtin;
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
    main: PathBuf,
  },
  #[command(about = "generate ast from fkl file")]
  Ast {
    #[arg(short, long)]
    main: PathBuf,
  },
  #[command(about = "generate code from fkl file")]
  Gen {
    #[arg(short, long, required = true)]
    main: PathBuf,
    #[arg(short, long = "impl")]
    impl_name: Option<String>,
  },
  #[command(about = "run function from fkl file")]
  Run(RunOpt),
}

#[derive(Debug, Args)]
struct RunOpt {
  /// main file of feakin
  #[arg(short, long, required = true)]
  main: PathBuf,
  /// the path of the function to run
  #[arg(short, long, required = false)]
  path: Option<PathBuf>,
  #[arg(short, required = false, long = "impl")]
  impl_name: Option<String>,
  #[arg(short, required = true, long = "func")]
  func_name: RunFuncName,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum RunFuncName {
  HttpRequest,
  Guarding,
}

// todo: add code highlight support
fn main() {
  env_logger::init_from_env(
    env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));

  let cli: Cli = Cli::parse();
  match &cli.command {
    Commands::Dot { main: path } => {
      gen_to_dot(path);
    }
    Commands::Ast { main: path } => {
      parse_to_ast(path);
    }
    Commands::Gen { main: path, impl_name } => {
      let parent = path.parent().unwrap().to_path_buf();
      code_gen_exec::code_gen_by_path(path, impl_name.clone(), &parent);
    }
    Commands::Run(run) => {
      let mir = mir_from_file(&run.main);
      info!("runOpt: {:?}", run);
      match run.func_name {
        RunFuncName::HttpRequest => {
          let impl_name = run.impl_name.as_ref().unwrap();
          builtin::endpoint_runner::execute(&mir, &run.func_name, &impl_name);
        }
        RunFuncName::Guarding => {
          let root = match &run.path {
            Some(path) => path.clone(),
            None => run.main.parent().unwrap().to_path_buf(),
          };

          let errors = LayeredGuardingExec::guarding(root, &mir.layered.expect("cannot parse guarding rule"));

          if errors.len() > 0 {
            for error in errors {
              println!("{}", error);
            }
          }
        }
      }
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

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use log::info;

use fkl_parser::mir::Environment;
use fkl_parser::parse;

pub mod construct;
pub mod code_meta;
pub mod inserter;
pub mod exec;
pub mod builtin;
pub mod highlighter;
mod e2e;
mod datasource;
pub mod mock;

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
  Gen(GenOpt),
  #[command(about = "run function from fkl file")]
  Run(RunOpt),
}

#[derive(Debug, Args)]
struct GenOpt {
  #[arg(short, long, required = true)]
  main: PathBuf,
  #[arg(short, long = "impl")]
  impl_name: Option<String>,
  #[arg(short, long = "framework", default_value = "spring")]
  framework: SupportedFramework,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum SupportedFramework {
  Spring,
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
  #[arg(short, required = false, long = "env")]
  env: Option<String>,
  #[arg(short, required = true, long = "func")]
  func_name: RunFuncName,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum RunFuncName {
  HttpRequest,
  Guarding,
  TestConnection,
  MockServer,
}

// todo: add app context for save highlighter
#[tokio::main]
async fn main() {
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
    Commands::Gen(opt) => {
      let parent = &opt.main.parent().unwrap().to_path_buf();
      exec::code_gen_runner::code_gen_by_path(&opt.main, opt.impl_name.clone(), &parent);
    }
    Commands::Run(run) => {
      let root = match &run.path {
        Some(path) => path.clone(),
        None => run.main.parent().unwrap().to_path_buf(),
      };

      let mir = exec::mir_from_file(&run.main);

      info!("runOpt: {:?}", run);
      match run.func_name {
        RunFuncName::HttpRequest => {
          let impl_name = run.impl_name.as_ref().unwrap();
          exec::endpoint_runner(&mir, &run.func_name, &impl_name);
        }
        RunFuncName::Guarding => {
          let layered = mir.layered.expect("layered architecture is required");
          exec::guarding_runner(root, &layered);
        }
        RunFuncName::TestConnection => {
          if mir.envs.len() == 0 {
            panic!("environment is required");
          }

          let env: &Environment = match &run.env {
            Some(env_name) => {
              mir.envs.iter()
                .filter(|env| &env.name == env_name)
                .collect::<Vec<&Environment>>()
                .first()
                .unwrap_or(&&mir.envs[0])
            }
            None => &mir.envs[0],
          };
          exec::test_connection_runner(&env).await;
        }
        RunFuncName::MockServer => {
          exec::mock_server_runner(&mir).await;
        }
      }
    }
  }
}

fn gen_to_dot(path: &PathBuf) {
  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  let context_map = parse(&*contents).expect("TODO: panic message");

  let json = serde_json::to_string(&context_map).expect("TODO: panic message");

  let mut file = std::fs::File::create("dot.dot").expect("TODO: panic message");
  file.write_all(json.as_bytes()).expect("TODO: panic message");
}


fn parse_to_ast(path: &PathBuf) {
  let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
  let context_map = parse(&*contents).expect("TODO: panic message");

  let json = serde_json::to_string(&context_map).expect("TODO: panic message");

  let mut file = fs::File::create("ast.json").expect("TODO: panic message");
  file.write_all(json.as_bytes()).expect("TODO: panic message");
}

#[cfg(test)]
mod tests {
  use indexmap::IndexMap;

  use fkl_codegen_java::gen_http_api;
  use fkl_parser::mir::{BoundedContext, ContextMap};
  use fkl_parser::mir::implementation::Implementation;
  use fkl_parser::parse;

  use crate::builtin::builtin_type::BuiltinType;
  use crate::exec::endpoint_runner;
  use crate::mock::fake_value::FakeValue;
  use crate::RunFuncName;

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

    endpoint_runner(&context_map, &RunFuncName::HttpRequest, "CinemaCreated");
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

    endpoint_runner(&context_map, &RunFuncName::HttpRequest, "CinemaCreated");
  }

  #[test]
  fn test_mir_struct() {
    let source = r#"ContextMap TicketBooking {
  TicketContext <-> ReservationContext;
}

Context TicketContext {
  Aggregate Ticket, Reservation;
}

Aggregate Ticket {
  Entity Ticket;
}

Entity Ticket {
  Struct {
    id: UUID;
    seat: String;
    price: Int;
  }
}
"#;

    let context_map: ContextMap = parse(source).unwrap();

    let contexts: Vec<BoundedContext> = context_map.contexts.iter()
      .filter(|context| context.name == "TicketContext")
      .map(|ctx| ctx.clone())
      .collect::<Vec<BoundedContext>>();

    let entity = contexts[0].aggregates[0].entities[0].clone();

    let fields = &entity.fields;
    let types = FakeValue::builtin_type(fields);

    assert_eq!(types.len(), 3);
    assert_eq!(types, IndexMap::from([
      ("id".to_string(), BuiltinType::Special("uuid".to_string())),
      ("seat".to_string(), BuiltinType::String),
      ("price".to_string(), BuiltinType::Integer),
    ]));
  }
}

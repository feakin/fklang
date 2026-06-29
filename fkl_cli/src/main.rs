use std::fs;
use std::io::Write;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use log::info;

use fkl_ext_loader::load_registry;
#[cfg(test)]
use fkl_mir::build_time_travel_trace;
use fkl_mir::{build_time_travel_trace_with_source, ContextMap, Environment, TimeTravelTrace, TimeTravelVariable};
use fkl_parser::parse;
use init::{init_project, InitOptions};

/// parse source code and generate MIR
pub mod deconstruct;
/// the MIR of source code
pub mod code_meta;
/// insert code to the source
pub mod inserter;
/// some built-in functions
pub mod builtin;
/// code highlight
pub mod highlighter;
mod e2e;
/// the database, datasource support
mod datasource;
/// mock server
pub mod mock;
/// generate feakin code
pub mod generator;
mod debug_adapter;
mod init;

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
  #[command(about = "print a time-travel debug trace for fkl flow steps")]
  Debug(DebugOpt),
  #[command(about = "start the fkl time-travel Debug Adapter Protocol server")]
  Dap(DapOpt),
  #[command(about = "initialize a new fkl project from a template")]
  Init(InitOpt),
  #[command(about = "list plugins from a local registry directory")]
  Plugin(PluginOpt),
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
  Sql,
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
  /// for example run for kafka
  ///```
  /// fkl_cli --func custom-function --env Local --custom kafka --main impl.fkl
  ///```
  #[arg(short, required = false, long = "custom")]
  custom_func: Option<String>,
}

#[derive(Debug, Args)]
struct DebugOpt {
  /// main file of feakin
  #[arg(short, long, required = true)]
  main: PathBuf,
  #[arg(short, long = "format", default_value = "text")]
  format: DebugOutputFormat,
}

#[derive(Debug, Args)]
struct DapOpt {
  /// main file of feakin. DAP launch requests may also provide this as `main` or `program`.
  #[arg(short, long, required = false)]
  main: Option<PathBuf>,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
enum DebugOutputFormat {
  Text,
  Json,
}

#[derive(Debug, Args)]
struct InitOpt {
  #[arg(short, long, default_value = "Demo")]
  name: String,
  #[arg(short, long, default_value = ".")]
  path: PathBuf,
  #[arg(short, long)]
  force: bool,
}

#[derive(Debug, Args)]
struct PluginOpt {
  #[arg(short, long, default_value = "plugins")]
  registry: PathBuf,
}

#[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
pub enum RunFuncName {
  HttpRequest,
  Guarding,
  TestConnection,
  EnvCheck,
  MockServer,
  CustomFunction,
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
      builtin::funcs::code_gen::code_gen_by_path(
        &opt.main,
        opt.impl_name.clone(),
        &parent,
        &opt.framework,
      );
    }
    Commands::Run(run) => {
      let root = match &run.path {
        Some(path) => path.clone(),
        None => run.main.parent().unwrap().to_path_buf(),
      };

      let mir = builtin::funcs::mir_from_file(&run.main);

      info!("runOpt: {:?}", run);
      match &run.func_name {
        RunFuncName::HttpRequest => {
          let impl_name = run.impl_name.as_ref().unwrap();
          builtin::funcs::endpoint_runner(&mir, &run.func_name, &impl_name);
        }
        RunFuncName::Guarding => {
          let layered = mir.layered.expect("layered architecture is required");
          builtin::funcs::guarding_runner(root, &layered);
        }
        RunFuncName::TestConnection => {
          if mir.envs.len() == 0 {
            panic!("environment is required");
          }

          let env = env_from_opt(&run, &mir);
          builtin::funcs::test_connection_runner(&env).await;
        }
        RunFuncName::EnvCheck => {
          if mir.envs.len() == 0 {
            panic!("environment is required");
          }

          let env = env_from_opt(&run, &mir);
          let checked = builtin::funcs::validate_environment_checks(&env)
            .unwrap_or_else(|err| panic!("{}", err));
          println!("checked {}", checked.join(","));
        }
        RunFuncName::MockServer => {
          builtin::funcs::mock_server_runner(&mir).await;
        }
        RunFuncName::CustomFunction => {
          let func_name = match &run.custom_func {
            Some(name) => name,
            None => panic!("custom function name is required"),
          };

          let env = env_from_opt(&run, &mir);
          builtin::funcs::custom_function_runner(&mir, &env, &func_name).await;
        }
      }
    }
    Commands::Debug(opt) => {
      let trace = debug_trace_from_file(&opt.main);
      match opt.format {
        DebugOutputFormat::Text => println!("{}", debug_trace_text(&trace)),
        DebugOutputFormat::Json => println!("{}", debug_trace_json(&trace)),
      }
    }
    Commands::Dap(opt) => {
      debug_adapter::run_stdio(opt.main.clone()).expect("failed to run debug adapter");
    }
    Commands::Init(opt) => {
      let main = init_project(InitOptions {
        name: opt.name.clone(),
        path: opt.path.clone(),
        force: opt.force,
      }).expect("failed to initialize fkl project");
      println!("created {}", main.display());
    }
    Commands::Plugin(opt) => {
      for plugin in load_registry(&opt.registry).expect("failed to load plugin registry") {
        println!("{}\t{:?}\t{}", plugin.name, plugin.kind, plugin.path.display());
      }
    }
  }
}

fn env_from_opt(run: &RunOpt, mir: &ContextMap) -> Environment {
  let env: &Environment = match &run.env {
    Some(env_name) => {
      mir.envs.iter()
        .filter(|env| &env.name == env_name)
        .collect::<Vec<&Environment>>()
        .first()
        .unwrap_or_else(|| panic!("cannot find environment: {}", env_name))
    }
    None => &mir.envs[0],
  };

  env.clone()
}

fn debug_trace_text(trace: &TimeTravelTrace) -> String {
  let mut output = String::from("# Time Travel Debug Trace\n");

  for frame in &trace.frames {
    output.push_str(&format!(
      "{} {} {} {}\n",
      frame.index, frame.implementation, frame.endpoint, frame.operation
    ));
    if let Some(source) = &frame.source {
      output.push_str(&format!(
        "  source: {}:{}:{}\n",
        source.path, source.start_line, source.start_column
      ));
    }
    if !frame.reads.is_empty() {
      output.push_str(&format!("  reads: {}\n", frame.reads.join(", ")));
    }
    if !frame.writes.is_empty() {
      output.push_str(&format!("  writes: {}\n", frame.writes.join(", ")));
    }
    output.push_str(&format!(
      "  state before: [{}]\n",
      format_debug_variables(&frame.state_before.variables)
    ));
    if !frame.state_diff.read.is_empty() {
      output.push_str(&format!(
        "  read: {}\n",
        format_debug_variables(&frame.state_diff.read)
      ));
    }
    if !frame.state_diff.created.is_empty() {
      output.push_str(&format!(
        "  created: {}\n",
        format_debug_variables(&frame.state_diff.created)
      ));
    }
    if !frame.state_diff.updated.is_empty() {
      output.push_str(&format!(
        "  updated: {}\n",
        format_debug_variables(&frame.state_diff.updated)
      ));
    }
    output.push_str(&format!(
      "  state after: [{}]\n",
      format_debug_variables(&frame.state_after.variables)
    ));
  }

  output
}

fn debug_trace_json(trace: &TimeTravelTrace) -> String {
  serde_json::to_string_pretty(trace).expect("failed to serialize debug trace")
}

#[cfg(test)]
fn debug_trace(mir: &ContextMap) -> TimeTravelTrace {
  build_time_travel_trace(&mir.implementations)
}

fn debug_trace_from_file(path: &PathBuf) -> TimeTravelTrace {
  let source = fs::read_to_string(path).expect("Something went wrong reading the file");
  let mir = parse(&source).expect("failed to parse fkl file");
  build_time_travel_trace_with_source(
    &mir.implementations,
    Some(&path.display().to_string()),
    Some(&source),
  )
}

#[cfg(test)]
fn debug_trace_text_for_source(path: &str, source: &str) -> String {
  let mir = parse(source).expect("failed to parse fkl source");
  let trace = build_time_travel_trace_with_source(
    &mir.implementations,
    Some(path),
    Some(source),
  );
  debug_trace_text(&trace)
}

fn format_debug_variables(variables: &[TimeTravelVariable]) -> String {
  variables
    .iter()
    .map(|variable| {
      let label = format!("{}:{}", variable.name, variable.type_name);
      match &variable.value {
        Some(value) => format!("{}={}", label, value),
        None => label,
      }
    })
    .collect::<Vec<String>>()
    .join(", ")
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
  use std::path::PathBuf;

  use indexmap::IndexMap;

  use fkl_codegen_java::gen_http_api;
  use fkl_mir::{BoundedContext, ContextMap};
  use fkl_mir::implementation::Implementation;
  use fkl_parser::parse;

  use crate::builtin::funcs::endpoint_runner;
  use crate::builtin::types::BuiltinType;
  use crate::mock::fake_value::FakeValue;
  use crate::RunFuncName;
  use crate::{debug_trace, debug_trace_text, debug_trace_text_for_source, Cli, Commands};
  use clap::Parser;

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
      ("id".to_string(), BuiltinType::Special("UUID".to_string())),
      ("seat".to_string(), BuiltinType::String),
      ("price".to_string(), BuiltinType::Integer),
    ]));
  }

  #[test]
  fn parses_debug_subcommand_for_time_travel_trace() {
    let cli = Cli::try_parse_from(["fkl", "debug", "--main", "docs/samples/impl.fkl"])
      .expect("debug command");

    match cli.command {
      Commands::Debug(opt) => {
        assert_eq!(opt.main, PathBuf::from("docs/samples/impl.fkl"));
      }
      _ => panic!("expected debug command"),
    }
  }

  #[test]
  fn parses_dap_subcommand_for_time_travel_debug() {
    let cli = Cli::try_parse_from(["fkl", "dap", "--main", "docs/samples/impl.fkl"])
      .expect("dap command");

    match cli.command {
      Commands::Dap(opt) => {
        assert_eq!(opt.main, Some(PathBuf::from("docs/samples/impl.fkl")));
      }
      _ => panic!("expected dap command"),
    }
  }

  #[test]
  fn renders_time_travel_debug_trace_as_text() {
    let source = r#"impl BookTicket {
  endpoint {
    POST "/tickets";
    response: Ticket;
  }

  flow {
    via UserRepository::getUserById receive user: User
    via TicketRepository::save(user: User) receive ticket: Ticket;
  }
}"#;
    let context_map: ContextMap = parse(source).unwrap();

    let trace = debug_trace(&context_map);
    let output = debug_trace_text(&trace);

    assert!(output.contains("# Time Travel Debug Trace"));
    assert!(output.contains("0 BookTicket POST /tickets UserRepository.getUserById"));
    assert!(output.contains("writes: user:User"));
    assert!(output.contains("1 BookTicket POST /tickets TicketRepository.save"));
    assert!(output.contains("reads: user:User"));
    assert!(output.contains("writes: ticket:Ticket"));
  }

  #[test]
  fn renders_time_travel_debug_trace_with_source_and_state() {
    let source = r#"impl BookTicket {
  endpoint {
    POST "/tickets";
    response: Ticket;
  }

  flow {
    via UserRepository::getUserById receive user: User
  }
}"#;

    let output = debug_trace_text_for_source("booking.fkl", source);

    assert!(output.contains("source: booking.fkl:8:5"));
    assert!(output.contains("state before: []"));
    assert!(output.contains("created: user:User=UserRepository.getUserById#0"));
    assert!(output.contains("state after: [user:User=UserRepository.getUserById#0]"));
  }
}

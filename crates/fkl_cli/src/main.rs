use std::fs;
use std::io::Write;
use std::path::PathBuf;

use fkl_parser::parse;

fn main() {
  let cmd = clap::Command::new("fkl")
    .bin_name("fkl")
    .subcommand_required(true)
    .subcommand(
      clap::command!("gen").arg(
        clap::arg!(--"path" <PATH>)
          .value_parser(clap::value_parser!(std::path::PathBuf)),
      ),
    );


  let matches = cmd.get_matches();
  let matches = match matches.subcommand() {
    Some(("gen", matches)) => matches,
    _ => unreachable!("clap should ensure we don't get here"),
  };

  let manifest_path = matches.get_one::<PathBuf>("path");
  match manifest_path {
    Some(path) => {
      gen_to_dot(path);
    }
    None => {
      println!("no path");
    }
  }
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

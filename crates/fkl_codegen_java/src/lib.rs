use genco::fmt;
use genco::prelude::*;

pub mod java_gen;
pub mod nlp;

pub use java_gen::entity_gen::*;
pub use java_gen::jpa_gen::*;
pub use java_gen::spring_code_gen::*;

use fkl_parser::mir::implementation::HttpApiImpl;

fn gen_http_api(_api: HttpApiImpl) -> anyhow::Result<()> {
  // let car = &java::import("com.feakin", "Car");
  // let list = &java::import("java.util", "List");
  // let array_list = &java::import("java.util", "ArrayList");

  let comment = "// This is a comment";

  let tokens: java::Tokens = quote! {
        $comment
        public static void main(String[] args) {

        }
    };

  let stdout = std::io::stdout();
  let mut w = fmt::IoWriter::new(stdout.lock());

  let fmt = fmt::Config::from_lang::<Java>().with_newline("\n");
  let config = java::Config::default().with_package("com.feakin");

  tokens.format_file(&mut w.as_formatter(&fmt), &config)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::implementation::HttpApiImpl;

  use crate::gen_http_api;

  #[test]
  fn basic_mir() {
    gen_http_api(HttpApiImpl::default()).unwrap();
  }
}

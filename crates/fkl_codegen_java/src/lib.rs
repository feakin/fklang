use genco::fmt;
use genco::prelude::*;

use fkl_parser::mir::Entity;

fn gen_entity_struct(entity: Entity) -> anyhow::Result<()> {
  let car = &java::import("com.feakin", "Car");
  let list = &java::import("java.util", "List");
  let array_list = &java::import("java.util", "ArrayList");

  let tokens: java::Tokens = quote! {
        public class HelloWorld {
            public static void main(String[] args) {
                $list<$car> cars = new $array_list<$car>();

                cars.add(new $car("Volvo"));
                cars.add(new $car("Audi"));

                for ($car car : cars) {
                    System.out.println(car);
                }
            }
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
  use fkl_parser::mir::Entity;
  use crate::gen_entity_struct;

  #[test]
  fn basic_mir() {
    gen_entity_struct(Entity::default()).unwrap();
  }
}

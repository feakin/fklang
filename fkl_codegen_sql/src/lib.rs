use std::collections::HashSet;

use fkl_mir::{ContextMap, Entity, Field};

pub fn gen_schema(mir: &ContextMap) -> String {
  let mut seen = HashSet::new();
  let mut tables = Vec::new();

  for context in &mir.contexts {
    for aggregate in &context.aggregates {
      for entity in &aggregate.entities {
        let table_name = to_snake_case(&entity.name);
        if seen.insert(table_name.clone()) {
          tables.push(gen_table(entity, &table_name));
        }
      }
    }
  }

  tables.join("\n")
}

fn gen_table(entity: &Entity, table_name: &str) -> String {
  let columns = entity
    .fields
    .iter()
    .map(gen_column)
    .collect::<Vec<String>>()
    .join(",\n");

  format!("CREATE TABLE {} (\n{}\n);\n", table_name, columns)
}

fn gen_column(field: &Field) -> String {
  format!(
    "  {} {}",
    to_snake_case(&field.name),
    sql_type(&field.type_type)
  )
}

fn sql_type(fkl_type: &str) -> &'static str {
  match fkl_type {
    "UUID" => "UUID",
    "Int" | "Integer" => "INTEGER",
    "Long" => "BIGINT",
    "Float" | "Double" => "REAL",
    "BigDecimal" | "Decimal" => "DECIMAL",
    "Boolean" | "Bool" => "BOOLEAN",
    "String" => "TEXT",
    _ => "TEXT",
  }
}

fn to_snake_case(input: &str) -> String {
  let mut output = String::new();
  let mut previous_is_lower_or_digit = false;

  for ch in input.chars() {
    if ch.is_ascii_alphanumeric() {
      if ch.is_ascii_uppercase() {
        if previous_is_lower_or_digit && !output.ends_with('_') {
          output.push('_');
        }
        output.push(ch.to_ascii_lowercase());
        previous_is_lower_or_digit = false;
      } else {
        output.push(ch);
        previous_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
      }
    } else if !output.is_empty() && !output.ends_with('_') {
      output.push('_');
      previous_is_lower_or_digit = false;
    }
  }

  output.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
  use fkl_mir::{Aggregate, BoundedContext, ContextMap, Entity, Field};

  use crate::gen_schema;

  #[test]
  fn generates_create_table_for_entities() {
    let mut ticket = Entity::new("Ticket");
    ticket.fields = vec![
      Field {
        name: "id".to_string(),
        type_type: "UUID".to_string(),
        initializer: None,
      },
      Field {
        name: "seatName".to_string(),
        type_type: "String".to_string(),
        initializer: None,
      },
      Field {
        name: "price".to_string(),
        type_type: "Int".to_string(),
        initializer: None,
      },
    ];

    let mir = ContextMap {
      name: "Ticketing".to_string(),
      contexts: vec![BoundedContext {
        name: "Sales".to_string(),
        aggregates: vec![Aggregate {
          name: "TicketSale".to_string(),
          entities: vec![ticket],
          ..Default::default()
        }],
      }],
      ..Default::default()
    };

    assert_eq!(
      gen_schema(&mir),
      "CREATE TABLE ticket (\n  id UUID,\n  seat_name TEXT,\n  price INTEGER\n);\n"
    );
  }
}

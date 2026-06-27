use fkl_mir::{Entity, Field};

pub struct EntityGen {}

pub fn gen_spring_entity(entity: &Entity, package: &str) -> String {
  let mut imports = vec![
    "import jakarta.persistence.Entity;".to_string(),
    "import jakarta.persistence.Id;".to_string(),
  ];

  if uses_uuid(entity) {
    imports.push("import java.util.UUID;".to_string());
  }

  let fields = entity
    .fields
    .iter()
    .map(|field| gen_field(field, is_id_field(entity, field)))
    .collect::<Vec<String>>()
    .join("\n");

  format!(
    "package {};\n\n{}\n\n@Entity\npublic class {} {{\n{}\n}}\n",
    package,
    imports.join("\n"),
    entity.name,
    fields
  )
}

fn gen_field(field: &Field, is_id: bool) -> String {
  let line = format!("  private {} {};", java_type(&field.type_type), field.name);
  if is_id {
    format!("  @Id\n{}", line)
  } else {
    line
  }
}

fn is_id_field(entity: &Entity, field: &Field) -> bool {
  if !entity.identify.name.is_empty() {
    return entity.identify.name == field.name;
  }

  field.name == "id"
}

fn uses_uuid(entity: &Entity) -> bool {
  entity
    .fields
    .iter()
    .any(|field| java_type(&field.type_type) == "UUID")
}

fn java_type(fkl_type: &str) -> &str {
  match fkl_type {
    "UUID" | "Uuid" | "uuid" => "UUID",
    "String" | "string" => "String",
    "Int" | "Integer" | "int" | "integer" => "Integer",
    "Long" | "long" => "Long",
    "Float" | "float" => "Float",
    "Double" | "double" => "Double",
    "Boolean" | "Bool" | "boolean" | "bool" => "Boolean",
    other => other,
  }
}

#[cfg(test)]
mod tests {
  use fkl_mir::{Entity, Field};

  use crate::spring_gen::entity_gen::gen_spring_entity;

  #[test]
  fn generates_spring_jpa_entity_from_mir_entity() {
    let mut ticket = Entity::new("Ticket");
    ticket.identify = Field {
      name: "id".to_string(),
      type_type: "UUID".to_string(),
      initializer: None,
    };
    ticket.fields = vec![
      ticket.identify.clone(),
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

    assert_eq!(
      gen_spring_entity(&ticket, "com.example.domain"),
      r#"package com.example.domain;

import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import java.util.UUID;

@Entity
public class Ticket {
  @Id
  private UUID id;
  private String seatName;
  private Integer price;
}
"#
    );
  }
}

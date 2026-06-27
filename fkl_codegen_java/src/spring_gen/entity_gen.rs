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

  let constructor = format!("  public {}() {{\n  }}", entity.name);
  let accessors = entity
    .fields
    .iter()
    .map(gen_accessors)
    .collect::<Vec<String>>()
    .join("\n\n");

  let body = vec![fields, constructor, accessors]
    .into_iter()
    .filter(|section| !section.is_empty())
    .collect::<Vec<String>>()
    .join("\n\n");

  format!(
    "package {};\n\n{}\n\n@Entity\npublic class {} {{\n{}\n}}\n",
    package,
    imports.join("\n"),
    entity.name,
    body
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

fn gen_accessors(field: &Field) -> String {
  let field_type = java_type(&field.type_type);
  let suffix = method_suffix(&field.name);
  format!(
    "  public {} get{}() {{\n    return {};\n  }}\n\n  public void set{}({} {}) {{\n    this.{} = {};\n  }}",
    field_type,
    suffix,
    field.name,
    suffix,
    field_type,
    field.name,
    field.name,
    field.name
  )
}

fn method_suffix(name: &str) -> String {
  let mut chars = name.chars();
  match chars.next() {
    Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
    None => String::new(),
  }
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

  public Ticket() {
  }

  public UUID getId() {
    return id;
  }

  public void setId(UUID id) {
    this.id = id;
  }

  public String getSeatName() {
    return seatName;
  }

  public void setSeatName(String seatName) {
    this.seatName = seatName;
  }

  public Integer getPrice() {
    return price;
  }

  public void setPrice(Integer price) {
    this.price = price;
  }
}
"#
    );
  }

  #[test]
  fn generates_accessors_for_spring_jpa_entity() {
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
    ];

    let output = gen_spring_entity(&ticket, "com.example.domain");

    assert!(output.contains("  public Ticket() {\n  }\n"));
    assert!(output.contains("  public UUID getId() {\n    return id;\n  }\n"));
    assert!(output.contains("  public void setId(UUID id) {\n    this.id = id;\n  }\n"));
    assert!(output.contains("  public String getSeatName() {\n    return seatName;\n  }\n"));
    assert!(output.contains("  public void setSeatName(String seatName) {\n    this.seatName = seatName;\n  }\n"));
  }
}

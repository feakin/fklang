use log::info;

use fkl_mir::{Datasource, Environment};

use crate::datasource::mysql_connector::MysqlConnector;
use crate::datasource::postgres_connector::PostgresConnector;

pub(crate) async fn test_connection_runner(env: &Environment) {
  info!("test connection: {:?}", env);
  match &env.datasources[0] {
    Datasource::MySql(mysql) => {
      MysqlConnector::new(mysql.clone())
        .await
        .unwrap_or_else(|| panic!("cannot create connector"))
        .test_connection().await;
    }

    Datasource::Postgres(pgsql) => {
      PostgresConnector::new(pgsql.clone())
        .await
        .unwrap_or_else(|| panic!("cannot create connector"))
        .test_connection().await;
    }
  }
}

pub(crate) fn validate_environment_checks(env: &Environment) -> Result<Vec<String>, String> {
  let mut passed = Vec::new();

  for check in &env.checks {
    let target = if check.target.is_empty() {
      check.name.as_str()
    } else {
      check.target.as_str()
    };
    let configured = match target {
      "datasource" | "database" => !env.datasources.is_empty(),
      "server" => true,
      custom => env.customs.iter().any(|env| env.name == custom),
    };

    if !configured {
      return Err(format!("check {} target {} is not configured", check.name, target));
    }

    passed.push(check.name.clone());
  }

  Ok(passed)
}

#[cfg(test)]
mod tests {
  use fkl_mir::{EnvironmentCheck, CustomEnv};

  use super::*;

  #[test]
  fn validate_environment_checks_rejects_missing_datasource() {
    let env = Environment {
      name: "Local".to_string(),
      checks: vec![
        EnvironmentCheck {
          name: "database".to_string(),
          target: "datasource".to_string(),
          attrs: vec![],
        }
      ],
      ..Default::default()
    };

    assert_eq!(
      validate_environment_checks(&env),
      Err("check database target datasource is not configured".to_string())
    );
  }

  #[test]
  fn validate_environment_checks_accepts_custom_target() {
    let env = Environment {
      name: "Local".to_string(),
      customs: vec![CustomEnv {
        name: "kafka".to_string(),
        attrs: vec![],
      }],
      checks: vec![
        EnvironmentCheck {
          name: "message_bus".to_string(),
          target: "kafka".to_string(),
          attrs: vec![],
        }
      ],
      ..Default::default()
    };

    assert_eq!(
      validate_environment_checks(&env),
      Ok(vec!["message_bus".to_string()])
    );
  }
}

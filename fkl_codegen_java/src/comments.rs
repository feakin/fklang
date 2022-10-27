use fkl_parser::mir::Step;

pub fn ai_comments(steps: &Vec<Step>) -> Vec<String> {
  steps.iter().enumerate().map(|(index, step)| {
    match step {
      Step::MethodCall(call) => {
        format!("// {}. {}", index + 1, call)
      }
      Step::Message(msg) => {
        format!("// {}. {}", index + 1, msg)
      }
      Step::RpcCall(_) => {
        "".to_string()
      }
    }
  }).collect()
}

#[cfg(test)]
mod tests {
  use fkl_parser::mir::{Message, MethodCall, Step, VariableDefinition};

  use crate::comments::ai_comments;

  #[test]
  fn format_ai_comments() {
    let comments = ai_comments(&vec![
      Step::MethodCall(MethodCall {
        name: "all".to_string(),
        object: "UserRepository".to_string(),
        method: "save".to_string(),
        parameters: vec![VariableDefinition {
          name: "user".to_string(),
          type_type: "User".to_string(),
          initializer: None,
        }],
        return_type: None,
      }),
      Step::Message(Message {
        from: "Content".to_string(),
        to: "".to_string(),
        topic: "sample:blabla".to_string(),
        message: "hello".to_string(),
      }),
    ]);

    assert_eq!(comments.join(" "), "// 1. call UserRepository.save with (user:User) // 2. send hello from Content to sample:blabla");
  }
}

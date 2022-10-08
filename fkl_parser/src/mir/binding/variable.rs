#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VariableDefinition {
    pub name: String,
    pub type_type: String,
    pub initializer: Option<String>,
}

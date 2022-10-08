use crate::mir::binding::VariableDefinition;
use crate::mir::flow::step::Step;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Flow {
    pub name: String,
    pub inline_doc: String,
    pub steps: Vec<Step>,
}




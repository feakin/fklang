use std::collections::HashMap;
use pest::Span;

// Todo: add Loc support
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy, Default)]
pub struct Loc(pub usize, pub usize);

impl Loc {
  pub(crate) fn from_pair(range: Span) -> Loc {
    Loc(range.start(), range.end())
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Identifier {
  pub name: String,
  pub loc: Loc,
}

// strategy DDD

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FklDeclaration {
  None,
  ContextMap(ContextMapDecl),
  BoundedContext(BoundedContextDecl),
  Domain(DomainDecl),
  Aggregate(AggregateDecl),
  Entity(EntityDecl),
  ValueObject(ValueObjectDecl),
  Implementation(ImplementationDecl),
  Component(ComponentDecl),
  Struct(StructDecl),
  DomainService(DomainServiceDecl),
  ApplicationService(ApplicationServiceDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UbiquitousLanguage {
  pub name: String,
  pub description: String,
  // use hashmap to make sure it's unique
  pub words: HashMap<String, UniqueWord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueWord {
  pub unique_name: String,
  pub display_name: String,
  pub context_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainDecl {
  pub name: String,
  pub description: String,
  pub sub_domains: Vec<SubDomain>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubDomain {
  pub name: String,
  pub subdomain_type: String,
  pub entities: Vec<BoundedContextDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextMapDecl {
  pub name: Identifier,
  pub contexts: Vec<BoundedContextDecl>,
  pub relations: Vec<ContextRelation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BoundedContextDecl {
  pub name: String,
  pub domain_events: Vec<DomainEventDecl>,
  pub aggregates: Vec<AggregateDecl>,
  pub used_domain_objects: Vec<UsedDomainObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DomainEventDecl {
  pub name: String,
  pub implementation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UsedImplementation {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextRelation {
  pub source: String,
  pub target: String,
  pub direction: RelationDirection,
  pub source_types: Vec<String>,
  pub target_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationDirection {
  Undirected,
  // -->
  PositiveDirected,
  // <--
  NegativeDirected,
  // <->
  BiDirected,
}

impl Default for RelationDirection {
  fn default() -> Self {
    RelationDirection::Undirected
  }
}

// tactic DDD

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainServiceDecl {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationServiceDecl {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AggregateDecl {
  pub name: String,
  pub inline_doc: String,
  pub used_domain_objects: Vec<UsedDomainObject>,
  pub entities: Vec<EntityDecl>,
  pub value_objects: Vec<ValueObjectDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StructDecl {
  pub name: String,
  pub inline_doc: String,
  pub fields: Vec<VariableDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UsedDomainObject {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainEvent {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntityDecl {
  pub name: String,
  pub is_aggregate_root: bool,
  pub identify: VariableDefinition,
  pub inline_doc: String,
  pub fields: Vec<VariableDefinition>,
  pub value_objects: Vec<ValueObjectDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VariableDefinition {
  pub name: String,
  pub type_type: String,
  pub initializer: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AttributeDefinition {
  pub key: String,
  pub value: String,
}

// ???
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
  pub required: bool,
  pub nullable: bool,
  pub unique: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValueObjectDecl {
  pub name: String,
  pub inline_doc: String,
  pub fields: Vec<VariableDefinition>,
}

// Binding To Function

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ImplementationDecl {
  pub name: String,
  pub inline_doc: String,
  pub qualified_name: String,
  // can be file path or url
  pub endpoints: Vec<EndpointDecl>,
  pub flows: Vec<FlowDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceSetDecl {
  pub name: String,
  pub inline_doc: String,
  pub file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EndpointDecl {
  pub name: String,
  pub method: String,
  pub uri: String,
  pub authorization: Option<AuthorizationDecl>,
  pub request: Option<HttpRequestDecl>,
  pub response: Option<HttpResponseDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthorizationDecl {
  pub authorization_type: String,
  pub username: Option<String>,
  pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpRequestDecl {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpResponseDecl {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FlowDecl {
  pub inline_doc: String,
  pub steps: Vec<StepDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepDecl {
  MethodCall(MethodCallDecl),
  Message(MessageDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MethodCallDecl {
  pub name: String,
  pub object: String,
  pub method: String,
  pub arguments: Vec<VariableDefinition>,
  pub return_type: Option<VariableDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageDecl {
  pub from: String,
  pub topic: String,
  pub message: String,
}

// Binding To Data

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trait {
  pub name: String,
  pub methods: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
  pub name: String,
  pub description: String,
  pub parameters: Vec<Parameter>,
  pub return_type: Vec<Parameter>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RestApi {
  pub name: String,
  pub method: HttpMethod,
  pub path: String,
  pub parameters: Vec<Parameter>,
  pub return_type: Vec<Parameter>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
  Get,
  Post,
  Put,
  Delete,
  Patch,
  Custom(String)
}

impl Default for HttpMethod {
  fn default() -> Self {
    HttpMethod::Get
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
  pub name: String,
  pub param_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComponentDecl {
  pub name: String,
  pub component_type: ComponentType,
  pub inline_doc: String,
  pub used_domain_objects: Vec<UsedDomainObject>,
  pub attributes: Vec<AttributeDefinition>,
}

// implementation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentType {
  Application,
  Service,
  Module,
  Package,
  //  or Classes ?
  Entities,
}

impl Default for ComponentType {
  fn default() -> Self {
    ComponentType::Application
  }
}

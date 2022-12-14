use std::collections::HashMap;
use pest::Span;

// todo: add Loc support
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
  Include(IncludeDecl),
  ContextMap(ContextMapDecl),
  BoundedContext(BoundedContextDecl),
  Aggregate(AggregateDecl),
  Entity(EntityDecl),
  ValueObject(ValueObjectDecl),
  Implementation(ImplementationDecl),
  Struct(StructDecl),
  // Domain(DomainDecl),
  Component(ComponentDecl),
  Layered(LayeredDecl),
  SourceSets(SourceSetsDecl),
  Env(EnvDecl),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeDecl {
  pub path: String,
  pub loc: Loc,
}

// todo: add support for unique
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UbiquitousLanguage {
  pub name: String,
  pub description: String,
  pub words: HashMap<String, UniqueWord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueWord {
  pub unique_name: String,
  pub display_name: String,
  pub context_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContextMapDecl {
  pub name: Identifier,
  pub contexts: Vec<BoundedContextDecl>,
  pub relations: Vec<ContextRelation>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BoundedContextDecl {
  pub name: String,
  pub domain_events: Vec<DomainEventDecl>,
  pub aggregates: Vec<AggregateDecl>,
  pub used_domain_objects: Vec<UsedDomainObject>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DomainEventDecl {
  pub name: String,
  pub loc: Loc,
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
  pub domain_events: Vec<DomainEventDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StructDecl {
  pub name: String,
  pub inline_doc: String,
  pub fields: Vec<VariableDefinition>,
  pub loc: Loc
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UsedDomainObject {
  pub name: String,
  pub loc: Loc
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntityDecl {
  pub name: String,
  pub is_aggregate_root: bool,
  pub identify: VariableDefinition,
  pub inline_doc: String,
  pub fields: Vec<VariableDefinition>,
  pub value_objects: Vec<ValueObjectDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VariableDefinition {
  pub name: String,
  pub type_type: String,
  pub initializer: Option<String>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AttributeDefinition {
  pub key: String,
  pub value: Vec<String>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValueObjectDecl {
  pub name: String,
  pub inline_doc: String,
  pub fields: Vec<VariableDefinition>,
  pub loc: Loc,
}

// Implementation Block

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ImplementationDecl {
  pub name: String,
  pub inline_doc: String,
  pub qualified_name: String,
  // can be file path or url
  pub endpoint: EndpointDecl,
  pub target: Option<ImplementationTarget>,
  pub flow: Option<FlowDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ImplementationTarget {
  pub target_type: ImplementationTargetType,
  pub name: String,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationTargetType {
  None,
  Aggregate,
  Entity,
  ValueObject,
}

impl Default for ImplementationTargetType {
  fn default() -> Self {
    ImplementationTargetType::None
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceSetsDecl {
  pub name: String,
  pub source_sets: Vec<SourceSetDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceSetDecl {
  pub name: String,
  pub attributes: Vec<AttributeDefinition>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EndpointDecl {
  pub name: String,
  pub method: String,
  pub uri: String,
  pub authorization: Option<AuthorizationDecl>,
  pub request: Option<HttpRequestDecl>,
  pub response: Option<HttpResponseDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthorizationDecl {
  pub auth_type: String,
  pub username: Option<String>,
  pub password: Option<String>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpRequestDecl {
  pub name: String,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpResponseDecl {
  pub name: String,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FlowDecl {
  pub inline_doc: String,
  pub steps: Vec<StepDecl>,
  pub loc: Loc,
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
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageDecl {
  pub from: String,
  pub topic: String,
  pub message: String,
  pub loc: Loc,
}

// Binding block

/// [`SourceSet`] is a code block is a block of code that can be executed.
/// It can be a function, a method, a class, a trait, a module, a file, a package, a library, a program, etc.
/// - [`source_type`]: the type of the source code, e.g. feakin, java, puml/uml, swagger, etc.
/// - [`src_dir`]: the directory of the source code
/// - [`filter`]: the filter of the source code
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceSet {
  pub name: String,
  /// default to be Feakin?
  pub source_type: String,
  pub src_dirs: Vec<String>,
  pub filter: String,
  pub loc: Loc,
}

// Layered Block

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LayeredDecl {
  pub name: String,
  pub inline_doc: String,
  pub dependencies: Vec<LayerRelationDecl>,
  pub layers: Vec<LayerDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LayerRelationDecl {
  pub source: String,
  pub target: String,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LayerDecl {
  pub name: String,
  pub inline_doc: String,
  pub package: String,
  pub loc: Loc,
}

// Architecture Binding Block

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
  pub loc: Loc,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EnvDecl {
  pub name: String,
  pub inline_doc: String,
  pub datasource: Option<DatasourceDecl>,
  pub server: Option<ServerDecl>,
  pub customs: Vec<CustomDecl>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DatasourceDecl {
  pub url: String,
  pub host: String,
  pub port: String,
  pub driver: String,
  pub username: String,
  pub password: String,
  pub database: String,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ServerDecl {
  pub port: u16,
  pub attributes: Vec<AttributeDefinition>,
  pub loc: Loc,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CustomDecl {
  pub name: String,
  pub inline_doc: String,
  pub attributes: Vec<AttributeDefinition>,
  pub loc: Loc,
}

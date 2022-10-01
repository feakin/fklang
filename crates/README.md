# Fkl - Feakin Language

> Feakin is enterprise architecture knowledge information notation.

Design Philosophy

- Architecture Twin. two-way binding for architecture.
- Lightweight Architecture Description.
- Event-based code generation. eventful ??
  - such as: `via xxEntity send xxMessage` 

## Modules

- domain_derive. a derive macro for domain types
- Parser
  - fkl_parser. parser for fkl.
  - fkl_parser_wasm. wasm wrapper version of fkl_parser.
- CLI. CLI for generator code and IDE support.
  - fkl_cli. the cli for fkl, like code_gen, dot_gen or others.
- LSP. language server protocol, for IDE/Editor Support
  - fkl_lsp. language server for fkl.
- CodeGen. code generator for fkl. 
  - fkl_codegen_dot. generate Graphviz dot language from fkl source.
  - fkl_codegen_java. generate Java code from fkl source.
    - [ ] spring boot
  - code_gen_plugins. generate for 
    - [ ] plugin interface
    - [ ] plugin for AWS Lambda

## DSL Design

- [ ] DDD Model
- [ ] DSL Parser
  - [ ] DSL Syntax
  - [ ] Ast Model
- [ ] Code Binding
- [ ] Code Generator Model
- [ ] Workflow DSL
  - [ ] Event Storming
    - [ ] Role
    - [ ] Command
    - [ ] Event

## EA

enterprise architecture, technology architecture

- Enterprise modeling
  - System Level
  - Application Level
  - Org. Modeling
    - Team Topology Model
- DDD Building Blocks
  - ContextMap
  - Bounded Context
    - Shared Kernel
    - Anti-corruption Layer
  - SubDomain
    - Core-domain
    - Supporting-domain
    - Generic-domain
- Software Implementation
  - Layered Architecture
    - Domain
    - Application
    - Infrastructure
    - Interface
  - Infrastructure
    - Cloud
    - On-premise

# Fklang

[![Build](https://github.com/feakin/fklang/actions/workflows/build.yml/badge.svg)](https://github.com/feakin/fklang/actions/workflows/build.yml)
![Crates.io](https://img.shields.io/crates/v/fkl_cli)
[![codecov](https://codecov.io/gh/feakin/fklang/branch/master/graph/badge.svg?token=PCIL3T0NDR)](https://codecov.io/gh/feakin/fklang)

> Fklang 是一个架构实现与设计双向绑定（架构孪生） 的 DSL。通过显性化软件架构设计，以确保软件系统描述与实现的一致性。并在工作流中，内嵌对于 AI 代码生成软件的支持，以构筑完整的开发者体验。

Try it: [Feakin Quick Start](https://book.feakin.com/quick-start.html)

Spec: [Feakin Specification](https://book.feakin.com/fklang/specification.html)

## Roadmap

1. DSL binding/integration
    - [ ] with Spring
2. contract base testing
    - [ ] with HTTP API
3. database integration: JPA, JDBC, etc.
    - [ ] database schema generation
    - [ ] database migration
4. module support
    - [ ] module dependency
    - [ ] module versioning
5. bootstrapping DDD DSL
    - [ ] use type system to describe domain model
6. build system inside: cache, incremental build, etc.
    - [ ] better code generation
7. general programming language (if possible)
    - [ ] function support
    - [ ] expression support
  

## Modules

- Parser
  - fkl_parser. parser for fkl.
  - fkl_parser_wasm. wasm wrapper version of fkl_parser.
- CLI. CLI for generator code and IDE support.
  - fkl_cli. the cli for fkl, like code_gen, dot_gen or others.
- LSP. language server protocol, for IDE/Editor Support
  - [ ] fkl_lsp. language server for fkl.
- CodeGen. code generator for fkl.
  - fkl_codegen_dot. generate Graphviz dot language from fkl source.
  - fkl_codegen_java. generate Java code from fkl source.
    - [ ] spring boot
- Plugins
  - [ ] plugin system
  - code_gen_plugins.
    - [ ] plugin for AWS Lambda
  - [ ] Domain Binding 
    - [ ] domain_derive. a derive macro for domain types

## License

@2022 This code is distributed under the MPL license. See `LICENSE` in this directory.

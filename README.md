# Fklang

[![Build](https://github.com/feakin/fklang/actions/workflows/build.yml/badge.svg)](https://github.com/feakin/fklang/actions/workflows/build.yml)
![Crates.io](https://img.shields.io/crates/v/fkl_cli)
[![codecov](https://codecov.io/gh/feakin/fklang/branch/master/graph/badge.svg?token=PCIL3T0NDR)](https://codecov.io/gh/feakin/fklang)

origin idea from [Forming](https://github.com/inherd/forming), but it's too complex. So I decide to rewrite it with
TypeFlow

> Fklang，一个基于[软件开发工业化](https://book.feakin.com/design-principles)思想设计的架构 DSL。
> 通过显式化的软件架构设计，以确保软件系统描述与实现的一致性，并探索结合 AI 代码生成。

Try it: [Feakin Quick Start](https://book.feakin.com/quick-start.html)

Spec: [Feakin Specification](https://book.feakin.com/fklang/specification.html)

## Install

```
cargo install fkl_cli
```

## Modules

- Parser
  - fkl_mir. the intermediate representation of fklang   
  - fkl_parser. parser for fkl.
  - fkl_parser_wasm. wasm wrapper version of fkl_parser.
- CLI. CLI for generator code and IDE support.
  - fkl_cli. the cli for fkl, like code_gen, dot_gen or others.
- LSP. language server protocol, for IDE/Editor Support
  - [x] fkl_lsp. language server for fkl.
  - [x] editors/vscode. VSCode extension that launches `fkl_lsp` over stdio.
- CodeGen. code generator for fkl.
  - fkl_codegen_dot. generate Graphviz dot language from fkl source.
  - fkl_codegen_java. generate Java code from fkl source.
  - fkl_codegen_sql. generate SQL schema from fkl source.
- Plugin System
  - fkl_ext_loader. load external plugins for fkl.
  - fkl_ext_api. the api for external plugins.
  - plugins
    - [x] ext_kafka. the plugin for kafka.
    - [x] ext_computing. the plugin for computing.
    - [x] ext_codegen_aws. the plugin for aws codegen.
    - [x] ext_sourceset_swagger. the plugin for swagger.

## Roadmap

- [x] DSL Design
- [x] IDEA Plugin
- Code gen
  - [x] with Spring
    - [x] Controller
    - [x] DDD
- contract base testing
  - [x] mock server
  - [x] with HTTP API
- database integration: JPA, JDBC, etc.
  - [x] database integration
    - [x] MySQL
    - [x] PostgresSQL
  - [x] database schema generation
  - [x] database migration
- plugin system
  - [x] plugin api
  - [x] plugin registry
- simple expr
  - [x] Expr
    - [x] logic expr
    - [x] math expr
    - etc.
  - [x] Filter
  - [x] REPL
  - [x] ExprTk with Rust?
- module support
  - [x] module dependency
  - [x] module versioning
- bootstrapping DDD DSL
  - [x] use type system to describe domain model
- build system inside: cache, incremental build, etc.
  - [x] better code generation
- general programming language (if possible)
  - [x] function support
  - [x] expression support
- IDE support
  - [x] VSCode LSP extension

## License

@2022 This code is distributed under the MPL license. See `LICENSE` in this directory.

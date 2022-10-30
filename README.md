# Fklang

[![Build](https://github.com/feakin/fklang/actions/workflows/build.yml/badge.svg)](https://github.com/feakin/fklang/actions/workflows/build.yml)
![Crates.io](https://img.shields.io/crates/v/fkl_cli)
[![codecov](https://codecov.io/gh/feakin/fklang/branch/master/graph/badge.svg?token=PCIL3T0NDR)](https://codecov.io/gh/feakin/fklang)

origin idea from [Forming](https://github.com/inherd/forming), but it's too complex. So I decide to rewrite it with TypeFlow

> Fklang 是一个基于[软件开发工业化](https://book.feakin.com/design-principles)思想，设计的架构设计 DSL。以确保软件系统描述与实现的一致性。通过显式化的软件架构设计，用于支持 AI 代码生成系统的嵌入

Try it: [Feakin Quick Start](https://book.feakin.com/quick-start.html)

Spec: [Feakin Specification](https://book.feakin.com/fklang/specification.html)

## Roadmap

1. DSL Design
2. IDEA Plugin
3. Code gen
    - [ ] with Spring
4. contract base testing
    - [ ] mock server
    - [ ] with HTTP API
5. database integration: JPA, JDBC, etc.
    - [ ] database integration
      - [ ] MySQL
      - [ ] PostgresSQL
      - [ ] SQLite
    - [ ] database schema generation
    - [ ] database migration
6. module support
    - [ ] module dependency
    - [ ] module versioning
7. bootstrapping DDD DSL
    - [ ] use type system to describe domain model
8. build system inside: cache, incremental build, etc.
    - [ ] better code generation
9. general programming language (if possible)
    - [ ] function support
    - [ ] expression support


## Install

```
cargo install fkl_cli
```

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

# Fklang

[![Build](https://github.com/feakin/fklang/actions/workflows/build.yml/badge.svg)](https://github.com/feakin/fklang/actions/workflows/build.yml)
![Crates.io](https://img.shields.io/crates/v/fklang)
[![codecov](https://codecov.io/gh/feakin/fklang/branch/master/graph/badge.svg?token=PCIL3T0NDR)](https://codecov.io/gh/feakin/fklang)

> Fklang 是 Feakin 提供的一个架构设计的 DSL，确保软件系统描述与实现的一致性。通过显式化的软件架构设计，用于支持 AI 代码生成系统的嵌入。

Try it: [Feakin Quick Start](https://book.feakin.com/quick-start.html)

Spec: [Feakin Specification](https://book.feakin.com/fklang/specification.html)

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

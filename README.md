# Fklang

>  Fklang 是 Feakin 提供的一个架构设计的 DSL，用于描述软件系统的架构 —— 确保软件系统描述与实现的一致性。

## Todos

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

## License

@2022 This code is distributed under the MPL license. See `LICENSE` in this directory.


Generate and test json conversion
```
cargo run --bin json_to_rust /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig > src/bin/example_generated.rs

cargo run --bin example_generated
```

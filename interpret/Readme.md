pass it a stack of json
each one will be applied to the previous

test like this
```
cargo run ~/2024/08/20/self-similar/ocaml-rust-unification/src/ocaml_types.json ~/2024/08/20/self-similar/ocaml-rust-unification/src/ocaml_types.json  | jq .

```

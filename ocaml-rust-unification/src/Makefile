t:
	cargo build

t2:
	RUST_BACKTRACE=full cargo run --bin ocaml-rust-unification  /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig

gdb:
	rust-gdb --args target/debug/ocaml-rust-unification /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig

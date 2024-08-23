
Generate and test json conversion
```
cargo run --bin json_to_rust /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig > src/bin/example_generated.rs

cargo run --bin example_generated
```



```gron /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig | cut -d= -f1 | sed -e's![0-9]!N!g' | sort | uniq -c | sort -n```

Results

|count| path|
------|------|
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_desc[N].pexp_loc_stack   |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc                      |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_end              |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_end.pos_bol      |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_end.pos_cnum     |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_end.pos_fname    |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_end.pos_lnum     |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_ghost            |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_start            |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_start.pos_bol    |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_start.pos_cnum   |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_start.pos_fname  |
| 98  | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_loc.loc_start.pos_lnum   |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args                                                           |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes                                                     |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc                                                            |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_end                                                    |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_end.pos_bol                                            |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_end.pos_cnum                                           |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_end.pos_fname                                          |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_end.pos_lnum                                           |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_ghost                                                  |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_start                                                  |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_start.pos_bol                                          |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_start.pos_cnum                                         |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_start.pos_fname                                        |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_loc.loc_start.pos_lnum                                         |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name                                                           |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN                                                      |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_end                                              |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_end.pos_bol                                      |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_end.pos_cnum                                     |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_end.pos_fname                                    |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_end.pos_lnum                                     |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_ghost                                            |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_start                                            |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_start.pos_bol                                    |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_start.pos_cnum                                   |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_start.pos_fname                                  |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.locN.loc_start.pos_lnum                                   |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_name.txtN                                                      |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_res                                                            |
| 109 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_vars                                                           |
| 113 | json[N].psig_desc[N][NN].ptype_kind[N]                                                                       |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_attributes                                                     |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc                                                            |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_end                                                    |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_end.pos_bol                                            |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_end.pos_cnum                                           |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_end.pos_fname                                          |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_end.pos_lnum                                           |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_ghost                                                  |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_start                                                  |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_start.pos_bol                                          |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_start.pos_cnum                                         |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_start.pos_fname                                        |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_loc.loc_start.pos_lnum                                         |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_mutable                                                        |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_mutable[N]                                                     |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name                                                           |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN                                                      |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_end                                              |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_end.pos_bol                                      |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_end.pos_cnum                                     |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_end.pos_fname                                    |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_end.pos_lnum                                     |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_ghost                                            |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_start                                            |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_start.pos_bol                                    |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_start.pos_cnum                                   |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_start.pos_fname                                  |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.locN.loc_start.pos_lnum                                   |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_name.txtN                                                      |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type                                                           |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_attributes                                           |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_desc                                                 |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc                                                  |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_end                                          |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_end.pos_bol                                  |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_end.pos_cnum                                 |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_end.pos_fname                                |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_end.pos_lnum                                 |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_ghost                                        |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_start                                        |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_start.pos_bol                                |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_start.pos_cnum                               |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_start.pos_fname                              |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc.loc_start.pos_lnum                               |
| 117 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_loc_stack                                            |
| 120 | json[N].psig_desc[N][NN].ptype_attributes[N].attr_payload[N][N].pstr_desc[N].pexp_desc[N][N]                 |
| 126 | json[N].psig_desc[N][N].ptype_kind[N][N].pcd_args[N][N].ptyp_desc[N]                                         |
| 135 | json[N].psig_desc[N][NN].ptype_kind[N][NN].pcd_attributes[N].attr_payload[N][N].pstr_desc[N]                 |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_attributes                                     |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_desc                                           |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc                                            |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_end                                    |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_end.pos_bol                            |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_end.pos_cnum                           |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_end.pos_fname                          |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_end.pos_lnum                           |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_ghost                                  |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_start                                  |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_start.pos_bol                          |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_start.pos_cnum                         |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_start.pos_fname                        |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc.loc_start.pos_lnum                         |
| 157 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_loc_stack                                      |
| 161 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N]                                                     |
| 180 | json[N].psig_desc[N][NN].ptype_kind[N][NN].pcd_attributes[N].attr_payload[N][N].pstr_desc[N].pexp_desc[N][N] |
| 196 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N]                                  |
| 196 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_desc[N].pexp_desc[N]     |
| 218 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N]                                                        |
| 222 | json[N].psig_desc[N][NN].ptype_kind[N][NN].pcd_args[N][N].ptyp_desc[N]                                       |
| 224 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_desc[N][N].ptyp_desc[N]                        |
| 226 | json[N].psig_desc[N][NN].ptype_kind[N][N]                                                                    |
| 294 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_desc[N]                  |
| 348 | json[N].psig_desc[N][NN].ptype_kind[N][N].pld_type.ptyp_desc[N]                                              |
| 392 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_attributes[N].attr_payload[N][N].pstr_desc[N].pexp_desc[N][N]  |
| 465 | json[N].psig_desc[N][NN].ptype_kind[N][N].pcd_args[N][N].ptyp_desc[N]                                        |


`grep -P "txt2" flatten.report | cut -d: -f1 | cut '-d"' -f2 | sed -e's/[0-9]//g' | sort -u`

..attr_name.txt

...pcd_args...pld_name.txt
...pcd_attributes..attr_name.txt
...pcd_name.txt

...pld_attributes..attr_name.txt
...pld_name.txt

...ptype_attributes..attr_name.txt

...ptype_kind...pcd_args...pld_name.txt
...ptype_kind...pcd_attributes..attr_name.txt
...ptype_kind...pcd_name.txt

...ptype_kind...pld_attributes..attr_name.txt
...ptype_kind...pld_name.txt

...ptype_name.txt

  
`sed -e's/[0-9]/_/g'  flatten.report| grep -P "\.+ptype_kind" | grep txt | grep -v ocaml.doc`

      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptop_def",
      _   "._._.ptype_kind._._.pld_name.txt_": "attr_name",
      _   "._._.ptype_kind._._.pld_name.txt_": "pconst_desc",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptop_dir",
      _   "._._.ptype_kind._._.pld_name.txt_": "attr_payload",
      _   "._._.ptype_kind._._.pld_name.txt_": "pconst_loc",
      _   "._._.ptype_kind._._.pld_name.txt_": "attr_loc",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Otag",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Oinherit",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ppat_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ppat_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ppat_loc_stack",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ppat_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_any",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_or",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_constraint",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_type",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_lazy",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_unpack",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_exception",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_effect",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_extension",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Ppat_open",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_var",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_alias",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_constant",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_interval",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_tuple",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_construct",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_variant",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_record",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ppat_array",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pexp_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pexp_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pexp_loc_stack",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pexp_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_ident",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_record",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_field",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_setfield",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_array",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_ifthenelse",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_sequence",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_while",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_for",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_constraint",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_coerce",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_constant",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_send",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_new",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_setinstvar",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_override",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_letmodule",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_letexception",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_assert",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_lazy",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_poly",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_object",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_let",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_newtype",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_pack",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_open",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_letop",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_extension",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pexp_unreachable",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_function",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_apply",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_match",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_try",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_tuple",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_construct",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pexp_variant",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pc_lhs",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pc_guard",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pc_rhs",
      _   "._.__.ptype_kind._._.pld_name.txt_": "let_",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ands",
      _   "._.__.ptype_kind._._.pld_name.txt_": "body",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pbop_op",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pbop_pat",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pbop_exp",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pbop_loc",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pparam_val",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pparam_newtype",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pparam_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pparam_desc",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pconst_integer",
      _   "._._.ptype_kind._._.pld_name.txt_": "pdir_name",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pconst_char",
      _   "._._.ptype_kind._._.pld_name.txt_": "pdir_arg",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pconst_string",
      _   "._._.ptype_kind._._.pld_name.txt_": "pdir_loc",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pconst_float",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pfunction_body",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pfunction_cases",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pconstraint",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcoerce",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pval_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pval_type",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pval_prim",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pval_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pval_loc",
	  
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_params",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_cstrs",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_kind",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_private",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_manifest",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_loc",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_abstract",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_variant",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_record",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_open",
	  
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_mutable",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_type",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_attributes",
	  
	  
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcd_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcd_vars",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcd_args",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcd_res",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcd_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcd_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcstr_tuple",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcstr_record",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyext_path",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyext_params",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyext_constructors",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyext_private",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyext_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyext_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pext_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pext_kind",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pext_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pext_attributes",
      _   "._._.ptype_kind._._.pld_name.txt_": "pdira_desc",
      _   "._._.ptype_kind._._.pld_name.txt_": "pdira_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyexn_constructor",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyexn_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptyexn_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pext_decl",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pext_rebind",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcty_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcty_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcty_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcty_constr",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcty_signature",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcty_arrow",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcty_extension",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcty_open",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcsig_self",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcsig_fields",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pctf_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pctf_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pctf_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pctf_inherit",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pctf_val",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pctf_method",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pctf_constraint",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pctf_attribute",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pctf_extension",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pci_virt",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pci_params",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pci_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pci_expr",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pci_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pci_attributes",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pdir_string",
      _   "._._.ptype_kind._._.pcd_name.txt_": "PStr",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pdir_int",
      _   "._._.ptype_kind._._.pcd_name.txt_": "PSig",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pdir_ident",
      _   "._._.ptype_kind._._.pcd_name.txt_": "PTyp",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Pdir_bool",
      _   "._._.ptype_kind._._.pcd_name.txt_": "PPat",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcl_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcl_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcl_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_constr",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_structure",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_fun",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_apply",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_let",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_constraint",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_extension",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcl_open",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcstr_self",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcstr_fields",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcf_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcf_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pcf_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_inherit",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_val",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_method",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_constraint",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_initializer",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_attribute",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pcf_extension",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Cfk_virtual",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Cfk_concrete",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmty_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmty_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmty_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_ident",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_signature",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_functor",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_with",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_typeof",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_extension",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmty_alias",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Unit",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Named",
      _   "._._.ptype_kind._._.pld_name.txt_": "ptyp_desc",
      _   "._._.ptype_kind._._.pld_name.txt_": "ptyp_loc",
      _   "._._.ptype_kind._._.pld_name.txt_": "ptyp_loc_stack",
      _   "._._.ptype_kind._._.pld_name.txt_": "ptyp_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "psig_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "psig_loc",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_value",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Psig_open",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Psig_include",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Psig_class",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Psig_class_type",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Psig_attribute",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Psig_extension",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_type",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_typesubst",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_typext",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_exception",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_module",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_modsubst",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_recmodule",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_modtype",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Psig_modtypesubst",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmd_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmd_type",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmd_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmd_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pms_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pms_manifest",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pms_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pms_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmtd_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmtd_type",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmtd_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmtd_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "popen_expr",
      _   "._.__.ptype_kind._._.pld_name.txt_": "popen_override",
      _   "._.__.ptype_kind._._.pld_name.txt_": "popen_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "popen_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pincl_mod",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pincl_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pincl_attributes",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_any",
      _   "._._.ptype_kind._.__.pcd_name.txt_": "Ptyp_package",
      _   "._._.ptype_kind._.__.pcd_name.txt_": "Ptyp_open",
      _   "._._.ptype_kind._.__.pcd_name.txt_": "Ptyp_extension",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_var",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_arrow",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_tuple",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_constr",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_object",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_class",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_alias",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_variant",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Ptyp_poly",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pwith_type",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pwith_module",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pwith_modtype",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pwith_modtypesubst",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pwith_typesubst",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pwith_modsubst",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmod_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmod_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmod_attributes",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_ident",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_structure",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_functor",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_apply",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_apply_unit",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_constraint",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_unpack",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pmod_extension",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pstr_desc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pstr_loc",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_eval",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pstr_class",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pstr_class_type",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pstr_include",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pstr_attribute",
      _   "._.__.ptype_kind._.__.pcd_name.txt_": "Pstr_extension",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_value",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_primitive",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_type",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_typext",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_exception",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_module",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_recmodule",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_modtype",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pstr_open",
      _   "._.__.ptype_kind._._.pcd_args._._.pld_name.txt_": "locally_abstract_univars",
      _   "._.__.ptype_kind._._.pcd_args._._.pld_name.txt_": "typ",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pvc_constraint",
      _   "._.__.ptype_kind._._.pcd_args._._.pld_name.txt_": "ground",
      _   "._.__.ptype_kind._._.pcd_args._._.pld_name.txt_": "coercion",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Pvc_coercion",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pvb_pat",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pvb_expr",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pvb_constraint",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pvb_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pvb_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmb_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmb_expr",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmb_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pmb_loc",
      _   "._._.ptype_kind._._.pld_name.txt_": "prf_desc",
      _   "._._.ptype_kind._._.pld_name.txt_": "prf_loc",
      _   "._._.ptype_kind._._.pld_name.txt_": "prf_attributes",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Rtag",
      _   "._._.ptype_kind._._.pcd_name.txt_": "Rinherit",
      _   "._._.ptype_kind._._.pld_name.txt_": "pof_desc",
      _   "._._.ptype_kind._._.pld_name.txt_": "pof_loc",
      _   "._._.ptype_kind._._.pld_name.txt_": "pof_attributes",  



`sed -e's/[0-9]/_/g'  flatten.report| grep -P "\.+ptype_kind" | grep txt | grep -v ocaml.doc`
And in that data set we find these:
	  	  	  
	  _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_params",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_cstrs",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_kind",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_private",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_manifest",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_attributes",
      _   "._.__.ptype_kind._._.pld_name.txt_": "ptype_loc",
	  
pld	  	  
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_name",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_mutable",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_type",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_loc",
      _   "._.__.ptype_kind._._.pld_name.txt_": "pld_attributes",

pcd
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_abstract",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_variant",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_record",
      _   "._.__.ptype_kind._._.pcd_name.txt_": "Ptype_open",



(.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f1 flatten.report | cut '-d"' -f2 | sort -n | uniq -c | sort -n

# level 1
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 pexp_loc_stack
      2 ptyp_loc_stack
    405 txt2
    581 ptyp_desc
    923 ptyp_loc
   2422 loc2

## level 2

      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 pexp_loc_stack
      2 ptyp_loc_stack
    403 txt2
   1671 loc_end
   1671 loc_start

cut -d. -f3 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 pexp_loc_stack
      2 ptyp_loc_stack
      9 popen_loc
     22 attr_name
    403 txt2
   1377 pexp_loc
   1380 attr_loc
   1578 pexp_desc
   4348 attr_payload
   


(.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f4 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 closed_flag
      1 direction_flag
      1 mutable_flag
      1 override_flag
      1 pcd_vars
      1 pexp_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_cstrs
      2 pexp_loc_stack
      2 ptyp_loc_stack
      4 arg_label
      8 pld_mutable
     18 loc2
     40 ptype_params
     71 ptype_private
    382 ptype_manifest
    403 txt2
    681 ptype_loc
    753 pld_attributes
    759 ptype_name
    870 pld_loc
   1011 pld_name
   1212 pcd_loc
   1337 pld_type
   1377 pstr_loc
   1396 pcd_name
   1817 ptyp_desc
   1821 ptype_attributes
   2069 loc_end
   2069 loc_start
   2956 pstr_desc
   3045 ptyp_loc
   3660 pcd_args
   5121 pcd_attributes
  23294 ptype_kind 
  
  
  
  (.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f5 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 override_flag
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 pexp_loc_stack
      2 ptyp_loc_stack
    403 txt2
    732 ptyp_desc
    987 ptyp_loc
   2763 loc2
   3527 loc_start
   3548 loc_end


(.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f6 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 mutable_flag
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_kind}[ptype_kind]}\n           and {{!type_declaration
      1 ptype_params
      2 closed_flag
      2 direction_flag
      2 override_flag
      2 pexp_loc_stack
      2 ptyp_loc_stack
      8 arg_label
    403 txt2
    770 attr_name
   1377 pexp_loc
   1380 pstr_loc
   1578 pexp_desc
   1581 attr_loc
   2495 loc_end
   2495 loc_start
   2966 pstr_desc
   5341 attr_payload  
  * 
  
  (.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f7 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 override_flag
      1 pcd_vars
      1 pexp_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 pexp_loc_stack
      2 ptyp_loc_stack
    139 pld_mutable
    403 txt2
    693 loc2
   1249 pld_attributes
   1263 pld_loc
   1404 pld_name
   1644 pcd_loc
   1828 pcd_name
   2095 ptyp_desc
   2108 pld_type
   2129 loc_end
   2129 loc_start
   3064 ptyp_loc
   4929 pcd_args
   8633 pcd_attributes
  * 
  
  (.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f8 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 closed_flag
      2 pexp_loc_stack
      2 ptyp_loc_stack
      3 override_flag
      8 arg_label
    403 txt2
   1134 ptyp_desc
   1380 pexp_loc
   1584 pexp_desc
   1611 ptyp_loc
   2907 loc2
   3084 loc_start
   3087 loc_end
   
   
   
   (.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f9 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 closed_flag
      1 direction_flag
      1 override_flag
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      1 ptyp_attributes
      1 ptype_attributes
      1 ptype_cstrs
      1 ptype_params
      2 mutable_flag
      2 pexp_loc_stack
      2 ptyp_loc_stack
      4 arg_label
   1581 pstr_loc
   1710 attr_loc
   1900 attr_name
   3397 loc_end
   3397 loc_start
   3683 pstr_desc
   6270 attr_payload


(.venv) mdupont@mdupont-G470:~/2024/08/20/self-similar/quasi-quote-json$ cut -d. -f10 flatten.report | cut '-d"' -f2 | grep -P '[a-zA-Z]' | sort -n | uniq -c | sort -n
      1 pcd_attributes
      1 pcd_vars
      1 pexp_attributes
      1 pld_attributes
      1 popen_attributes
      2 pexp_loc_stack
      2 ptyp_loc_stack
      3 rec_flag
      4 closed_flag
      4 override_flag
      4 pld_mutable
      7 arg_label
     36 pld_loc
     40 pld_name
     77 pld_type
    403 txt2
   1512 loc_end
   1512 loc_start
   1710 loc2
   2666 ptyp_desc
   3636 ptyp_loc

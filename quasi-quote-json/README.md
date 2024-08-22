
Generate and test json conversion
```
cargo run --bin json_to_rust /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig > src/bin/example_generated.rs

cargo run --bin example_generated
```

# example

```gron /time/2023/12/28/ppxlib/ocaml-opam-ppxlib-json-ast/ocaml/parsing/parsetree.mli.sig | cut -d= -f1 | sed -e's![0-9]!N!g' | sort | uniq -c | sort -n```

Results
| Count|path|
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

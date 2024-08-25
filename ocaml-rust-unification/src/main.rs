use std::env;
mod ocaml_types;
use crate::ocaml_types::Signature;
use std::fs::File;
fn main() {
    println!("Hello, world!");

    for argument in env::args_os().skip(1) {
        let json_file_path = argument;
        let s = json_file_path.into_string().unwrap();
        println!("read = {:?}", s);
        let deserialized: Signature = serde_json::from_reader(File::open(s).unwrap()).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
}

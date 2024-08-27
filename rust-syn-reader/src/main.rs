mod introspector;
use std::env;
use std::fs::File;
use introspector::syn::reader::types;

fn main() {
    println!("Hello, world!");

    for argument in env::args_os().skip(1) {
        let json_file_path = argument;
        let s = json_file_path.into_string().unwrap();
        println!("read = {:?}", s);
        let deserialized: introspector::syn::reader::types::Root = serde_json::from_reader(File::open(s).unwrap()).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
}

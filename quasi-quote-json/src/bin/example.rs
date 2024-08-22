#![recursion_limit = "13256"]
use std::fs::File;
use serde_json::Value;
//struct Module {}

// fn json_to_rust(x: Vec<Module>) -> bool
// {
//     return false;
// }

//#[derive(Debug, PartialEq,Clone)]
//struct Ret{}

fn find_object(_depth:usize,
	      _name1:&str,
	      _name2: &str,
	       _name3: &str,
	       _name4: &str,
	      _children: &Vec<bool>)->bool{
    return false
}
fn find_array(_depth:usize,
	      _index:usize,
	      _name: &str,
	      _name2: &str,
	      _children: &Vec<bool>)->bool{
    return false
}

fn check_schema(_root_item: &serde_json::Value) -> bool {
    let foo = find_array(0,0,"r","r__0_0",
			 &[
			     find_object(1,"r__0_0","psig","desc","r__0_0__psig_desc_1",
					 &[
					     find_array(2,0,"r__0_0__psig_desc_1","r__0_0__psig_desc_1__2_0",&[].to_vec()
					     ),
					     find_array(2,1,"r__0_0__psig_desc_1","r__0_0__psig_desc_1__2_1",
							&[
							    find_object(3,"r__0_0__psig_desc_1__2_1","attr","name","r__0_0__psig_desc_1__2_1__attr_name_3",&[].to_vec())
								
							].to_vec()
					     )
					 ].to_vec()
			     )
			 ].to_vec());
    return foo;
}


fn main() {
    let json_schema: Value = serde_json::from_reader(File::open("parsetree.mli.sig.json").unwrap()).unwrap();
    check_schema(&json_schema);
    
}


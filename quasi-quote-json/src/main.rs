extern crate serde_json;
use std::env;
use serde_json::Value;
use std::fs::File;

   
const SCHEMA : &'static [& 'static str] = &[
    "attr_name",
    "attr_payload",
    "pcd_args",
    "pcd_attributes",
    "pcd_name",
    "pcd_res",
    "pcd_vars",
    "pexp_attributes",
    "pexp_desc",
    "pld_attributes",
    "pld_mutable",
    "pld_name",
    "pld_type",
    "popen_attributes",
    "popen_expr",
    "popen_override",
    "psig_desc",
    "pstr_desc",
    "ptyp_attributes",
    "ptyp_desc",
//    "ptyp_loc_stack",
    "ptype_attributes",
    "ptype_cstrs",
    "ptype_kind",
    "ptype_manifest",
    "ptype_name",
    "ptype_params",
    "ptype_private"
    //    "attr_loc",
    //    "pcd_loc",
    //    "pexp_loc",
    //    "pexp_loc_stack",
    //    "pld_loc",
    //    "popen_loc",
    //    "psig_loc",
    //    "pstr_loc",
    //"pos_bol",
    //"pos_cnum",
    //"ptyp_loc",
    //"ptype_loc",
    // txt2
    //"pos_fname"
    //"pos_lnum"
    //    loc2
    // loc_end
    // loc_ghost
    // loc_start

];

fn top_level_search(item: &Value, depth: usize, inname: &str) {
    if let Value::Array(arr) = &item {
        for (i,item) in arr.into_iter().enumerate() {
	    let indent = "  ".repeat(depth);
	    let varname_item = format!("{}__{}_{}",inname,depth,i);	    
	    println!("{} tools::find_array({},{},\"{}\",\"{}\", &[",
		     indent,
		     depth,
		     i,
		     inname,
		     varname_item);
	    top_level_search(item, depth + 1, &varname_item);
	    println!("{} tools::end_array() ].to_vec()),",indent);
	}
    }    
    if let Value::Object(obj) = &item {
	for (key, value) in obj.iter() {
	    let k =key.to_string();
	    let k2 = k.as_str();
	    if SCHEMA.contains(&k2) {
		let indent = "  ".repeat(depth);
		let parts  = k.split("_").take(2).collect::<Vec<&str>>();
	        let outname = format!("{}__{}_{}_{}",inname,parts[0],parts[1],depth);
		println!("{} tools::find_object({},\"{}\",\"{}\",\"{}\",\"{}\",&[",indent,depth,inname,parts[0],parts[1],&outname);
		top_level_search(value, depth +1,&outname);
		println!("{} tools::end_object() ].to_vec()),",indent);
	    }
	}
    }
}

fn main() {
    for argument in env::args_os().skip(1) {
	let json_file_path = argument;
	let s = json_file_path.into_string().unwrap();
	let json_data: Value = serde_json::from_reader(File::open(s).unwrap()).unwrap();
	println!("fn main() {{");
	println!("let results = &[");
	top_level_search(&json_data,0,"r");
	println!("].to_vec();");
	println!("}}");
    }
}

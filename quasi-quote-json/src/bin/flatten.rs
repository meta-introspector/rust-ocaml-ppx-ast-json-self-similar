extern crate serde_json;
//use serde_json::json;
use serde_json::Value;
use std::env;
use std::fs::File;
extern crate flatten_json;
//use flatten_json::flatten;
use std::error::Error;
use std::collections::HashMap;

#[derive(serde::Deserialize,serde::Serialize, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Definition {
    ident: String,
    //body:  Value,
    inname: String,
    depth: usize
}

pub type Names = HashMap<String,Definition>;

#[derive(serde::Deserialize,
	 serde::Serialize, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Schema {
    items: Names,
}

fn top_level_search(
    schema: &mut Schema,
    item: &Value,
    depth: usize,
    inname: &str,
) -> Result<(), Box<dyn Error>> {
    //let indent = "  ".repeat(depth);
//    println!("{} DEBUG inname {} ,",indent,inname);
    //let item_val = format!("{}", serde_json::to_string_pretty(&item_val).unwrap());
    //schema.items.insert(ite.to_string(),Definition {
//	ident: item_val.to_string(),
//	depth: depth,
//	inname: inname.to_string(),
  //  });

    match item {
	Value::Array(arr) => {
            for (i, item) in arr.into_iter().enumerate() {
		//let indent = "  ".repeat(depth);
		let varname_item = format!("{}__{}_{}", inname, depth, i);
		//indent,
		//	     depth,
		//	     inname,
		//	     varname_item);
		let _ = top_level_search(schema,item, depth + 1, &varname_item);
		//println!("{} tools::end_array() ].to_vec()),",indent);
            }
	}
	Value::String(s) => {
	    //let s2 = s.to_string();
	    let long_name = format!("{}.{}", inname, s);
	    if schema.items.contains_key(&long_name) {
	    } else {
		schema.items.insert(long_name.to_string(),Definition {
		    ident: s.to_string(),
		    depth: depth,
		    inname: inname.to_string(),
		});
	    }	    
	}
	// None => {
	// }
	Value::Object(obj) => {
            for (key, value) in obj.iter() {
		let k = key.to_string();
		let k2 = k.as_str();
		let long_name = format!("{}.{}", inname, k2);
		let long_name2 = long_name.to_string();
		if schema.items.contains_key(&long_name2) {
		}
		else{
		    schema.items.insert(long_name.to_string(),Definition {
			ident: k2.to_string(),
			//body: value.clone(),
			depth: depth,
			inname: inname.to_string(),
		    });
		}

                //let indent = "  ".repeat(depth);
                let parts = k.split("_").take(2).collect::<Vec<&str>>();
		let pnames =  parts.join("_");		    
                let outname = format!("{}__{}_{}", inname, pnames, depth);
                let _ = top_level_search(schema,value, depth + 1, &outname);
                //let mut flat_value: Value = json!({});
                //let _ = flatten(value, &mut flat_value, None, true);
                //println!("{}", serde_json::to_string_pretty(&flat_value).unwrap());
	    } // else {
              //       //println!("adding schema {}", k2);
              //       schema.items.insert(k2.to_string(),Definition {
	      // 		ident: k2.to_string(),
	      // 		//body: value.clone(),
	      // 	    depth: depth,
	      // 		inname: inname.to_string(),
	    // 	    });
	    

	    
	},
	serde_json::Value::Null => {
	    //	    println!("unknown2 {:?}", item);
	    let long_name = format!("{} NONE", inname);
	    if schema.items.contains_key(&long_name) {
	    } else {
		schema.items.insert(long_name.to_string(),Definition {
		    ident: "None".to_string(),
		    depth: depth,
		    inname: inname.to_string(),
		});
	    }	    

	}
	_ => {
	    println!("unknown {:?}", item);
	},
    }
    return Ok(());
}

fn main() {
    for argument in env::args_os().skip(1) {
        let json_file_path = argument;
        let s = json_file_path.into_string().unwrap();
        let json_data: Value = serde_json::from_reader(File::open(s).unwrap()).unwrap();
        let mut s= Schema {
	    items: Names::new()
	};
        let _s2 = top_level_search(&mut s, &json_data, 0, "r").unwrap();
	println!("{}", serde_json::to_string_pretty(&s).unwrap());
    }
}

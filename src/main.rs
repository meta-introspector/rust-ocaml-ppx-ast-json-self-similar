use std::env;
use crate::serde_json::Value;
use serde_json;
use std::fs::File;
use std::io::Read;

fn search_values(obj: &serde_json::Value, search_string: &str) -> bool {
    match obj {
        serde_json::Value::Array(arr) => {
            for item in arr {
                if search_values(item, search_string)
		{
		    return true
		}

            }
	    return false
        }
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                if let serde_json::Value::String(val) = value {
                    if val.to_lowercase().find(search_string.to_lowercase().as_str()).is_some() {
                        println!("Found match: {} : {} = {}", search_string, key, val);
			return true
                    }
		    else
		    {
			(); // pass
		    }
                } else {
                    if search_values(value, search_string) {
			return true
		    }
	        }
            }
	    return false
        }
        serde_json::Value::String(val) => {
            if val.to_lowercase().find(search_string.to_lowercase().as_str()).is_some() {
		println!("Found match Str: {} IN {}", search_string, val);
		return true
            }
	    else
	    {
		return false
	    }
        }
        _ => false
    }
}
    
fn top_level_search(item: &Value) {
    if let Value::Array(arr) = &item {
        for item in arr {
	    top_level_search(item);
	}
    }    
    if let Value::Object(obj) = &item {
	for (key, value) in obj.iter() {
	    let k =key.to_string();

	    if search_values(value,k.as_str()) // search for k in the subobjects of value
	    {
		let v =value.to_string();
		println!("CONTAINS KEY={} VALUE={}",k,v);
	    }
	    top_level_search(value);
	}
    }
}
		       
fn self_similar_search(json_file_path: &str// , model_path: &str
) {
    let mut file = match File::open(json_file_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open JSON file: {}", err),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    top_level_search(&json);
}

fn main() {
    for argument in env::args_os().skip(1) {
	let json_file_path = argument;
	let s = json_file_path.into_string().unwrap();
	self_similar_search(&s);
    }
}

use std::env;
use crate::serde_json::Value;
use serde_json;
use serde_json::json;

//use serde::de::Error;
// use serde_json::json::Value;
use std::fs::File;
use std::io::Read;
// use std::collections::HashMap;
//use serde::{Deserialize, Serialize};

// Define a trait for the decoding functions
// trait DecodeFunction {
//     fn decode(&self, value: &Value) -> Result<(), serde_json::Error>;
// }

// Implement the decoding functions
// struct PsigOpenDecoder;
// struct PsigAttributeDecoder;
// struct PsigTypeDecoder;

// impl DecodeFunction for PsigOpenDecoder {
//     fn decode(&self, value: &Value) -> Result<(), serde_json::Error> {
//         // Implement the decoding logic for PsigOpen
//         println!("Decoding PsigOpen: {:?}", value);
//         Ok(())
//     }
// }

// impl DecodeFunction for PsigAttributeDecoder {
//     fn decode(&self, value: &Value) -> Result<(), serde_json::Error> {
//         // Implement the decoding logic for PsigAttribute
//         println!("Decoding PsigAttribute: {:?}", value);
//         Ok(())
//     }
// }

// impl DecodeFunction for PsigTypeDecoder {
//     fn decode(&self, value: &Value) -> Result<(), serde_json::Error> {
//         // Implement the decoding logic for PsigType
//         println!("Decoding PsigType: {:?}", value);
//         Ok(())
//     }
// }

// // Define a map from names to decoding functions
// fn get_decoding_functions() -> HashMap<String, Box<dyn DecodeFunction>> {
//     let mut map = HashMap::new();
//     map.insert("Psig_open".to_string(), Box::new(PsigOpenDecoder));
//     map.insert("Psig_attribute".to_string(), Box::new(PsigAttributeDecoder));
//     map.insert("Psig_type".to_string(), Box::new(PsigTypeDecoder));
//     map
// }


// // Define a trait for the psig desc types
// trait PsigDesc {
//     fn decode(value: &Value) -> Result<Self, serde_json::Error>
//     where
//         Self: Sized;
// }

// // Implement the psig desc types
// #[derive(Serialize, Deserialize, Debug)]
// struct PsigOpen;

// impl PsigDesc for PsigOpen {
//     fn decode(value: &Value) -> Result<Self, serde_json::Error> {
//         // Implement the decoding logic for PsigOpen
//         println!("Decoding PsigOpen: {:?}", value);
//         Ok(PsigOpen)
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct PsigAttribute;

// impl PsigDesc for PsigAttribute {
//     fn decode(value: &Value) -> Result<Self, serde_json::Error> {
//         // Implement the decoding logic for PsigAttribute
//         println!("Decoding PsigAttribute: {:?}", value);
//         Ok(PsigAttribute)
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct PsigType;

// impl PsigDesc for PsigType {
//     fn decode(value: &Value) -> Result<Self, serde_json::Error> {
//         // Implement the decoding logic for PsigType
//         println!("Decoding PsigType: {:?}", value);
//         Ok(PsigType)
//     }
// }



// trait PsigDescDecoder {
//     fn decode(&self, value: &Value) -> Result<Box<dyn PsigDesc>, serde_json::Error>;
// }

// struct PsigDescDecoderImpl<F> {
//     func: F,
// }

// impl<F> PsigDescDecoder for PsigDescDecoderImpl<F>
// where
//     F: Fn(&Value) -> Result<Box<dyn PsigDesc>, serde_json::Error>,
// {
//     fn decode(&self, value: &Value) -> Result<Box<dyn PsigDesc>, serde_json::Error> {
//         (self.func)(value)
//     }
// }

// fn get_psig_desc_types() -> HashMap<String, Box<dyn PsigDescDecoder>> {
//     let mut map = HashMap::new();
//     map.insert(
//         "Psig_open".to_string(),
//         Box::new(PsigDescDecoderImpl {
//             func: |value| PsigOpen::decode(value).map(|x| Box::new(x) as Box<dyn PsigDesc>),
//         }),
//     );
//     map.insert(
//         "Psig_attribute".to_string(),
//         Box::new(PsigDescDecoderImpl {
//             func: |value| PsigAttribute::decode(value).map(|x| Box::new(x) as Box<dyn PsigDesc>),
//         }),
//     );
//     map.insert(
//         "Psig_type".to_string(),
//         Box::new(PsigDescDecoderImpl {
//             func: |value| PsigType::decode(value).map(|x| Box::new(x) as Box<dyn PsigDesc>),
//         }),
//     );
//     map
// }



// // Define a map from names to psig desc types
// fn get_psig_desc_types() -> HashMap<String, Box<dyn Fn(&Value) -> Result<Box<dyn PsigDesc>, serde_json::Error>>> {
//     let mut map = HashMap::new();
//     map.insert(
//         "Psig_open".to_string(),
//         Box::new(|value| Ok(Box::new(PsigOpen::decode(value).unwrap()))
// 	)
	    
// fn get_psig_desc_types() -> HashMap<String, Box<dyn Fn(&Value) -> Result<Box<dyn PsigDesc>, serde_json::Error>>> {
//     let mut map = HashMap::new();
//     map.insert(
//         "Psig_open".to_string(),
//         Box::new(|value| PsigOpen::decode(value).map(|x| Box::new(x) as Box<dyn PsigDesc>)),
//     );
//     map.insert(
//         "Psig_attribute".to_string(),
//         Box::new(|value| PsigAttribute::decode(value).map(|x| Box::new(x) as Box<dyn PsigDesc>)),
//     );
//     map.insert(
//         "Psig_type".to_string(),
//         Box::new(|value| PsigType::decode(value).map(|x| Box::new(x) as Box<dyn PsigDesc>)),
//     );
//     map
// }


// // Dispatch the decoding function based on the name
//use serde_json::Error;
// fn decode_psig_desc(name: &str, value: &Value) -> Result<(), serde_json::Error> {
//     println!("decode_psig_desc: {}", name);    
//     match name {
//         "Psig_open" => PsigOpen::decode(value).map(|_| ()),
//         "Psig_attribute" => PsigAttribute::decode(value).map(|_| ()),
//         "Psig_type" => PsigType::decode(value).map(|_| ()),
// 	_ => unreachable!(),
//     }
// }

// fn print_psig_desc(psig_desc: Value) {
//     // Check if the value is an object
//     // if let Value::Object(obj) = &json {
//         // Check if the object has a key
//         // if let Some(psig_desc) = obj.get("psig_desc") {
//             // Check if psig_desc is an array
//             if let Value::Array(ref psig_desc_arr) = psig_desc {
//                 // Check if the array has at least three elements
//                 if psig_desc_arr.len() >= 2 {
//                     // Access the first element
//                     println!("psig_desc first element: {}", psig_desc_arr[0]);
// 		    let str = psig_desc_arr[0].as_str().unwrap();
// 		    let des = psig_desc.clone();
// 		    decode_psig_desc(
// 			&str,
// 			&des).unwrap();
		    
//                     // Access the second element
//                     //println!("psig_desc second element: {:?}", psig_desc_arr[1]);
		    
//                     // // Access the third element
//                     // if let Value::Array(third_elem_arr) = &psig_desc_arr[2] {
//                     //     // if let Value::Object(third_elem_obj) = &third_elem_arr[0] {
//                     //         // Access the txt2 value
//                     //     // println!("txt2: {}", third_elem_obj.get("txt2").unwrap());
// 		    // 	 println!("DEBUG: {}", third_elem_arr[0]);
//                     //     // }
//                     // }
//                 }
//             }
//         // }
//     // }
// }


fn foo(item: &Value) {
    if let Value::Array(arr) = &item {
        for item in arr {
	    foo(item);
	}
    }    
    if let Value::Object(obj) = &item {
	for (key, value) in obj.iter() {
	    let k =key.to_string();
	    let v =value.to_string();
	    if k.find(v.as_str()).is_some() {
		println!("k v: {} {}", k, v);
	    }	    
	    if v.find(k.as_str()).is_some() {
		println!("CONTAINS KEY={} VALUE={}",k,v);
	    }
	    //foo(k); k is String
	    foo(value);
	}
    }

    
}
		       
fn self_similar_search(json_file_path: &str// , model_path: &str
) -> serde_json::Value {
    let mut file = match File::open(json_file_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open JSON file: {}", err),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
//    let substring = &contents[..100.min(contents.len())];
//    println!("sample: {}", substring);
    
    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    foo(&json);
    
//     // Check if the value is an array
//     if let Value::Array(arr) = &json {
//         // Iterate over the array
//         for item in arr {
//             // Check if the item is an object
//             if let Value::Object(obj) = &item {
//                 // Check if the object has a key
//                 if let Some(psig_desc) = obj.get("psig_desc") {
// 	//	    print_psig_desc(psig_desc.clone());
		    
//                     // // Check if the psig_desc is an array
//                     // if let Value::Array(psig_desc_arr) = psig_desc {
//                     //     // Iterate over the psig_desc array
//                     //     for psig_desc_item in psig_desc_arr {
//                     //         // Check if the psig_desc item is an object
//                     //         if let Value::Object(psig_desc_obj) = &psig_desc_item {
//                     //             // Access the attr_loc object
//                     //             if let Some(attr_loc) = psig_desc_obj.get("attr_loc") {
//                     //                 // // Check if attr_loc is an object
//                     //                 // if let Value::Object(attr_loc_obj) = attr_loc {
//                     //                 //     // Access the loc_end object
//                     //                 //     if let Some(loc_end) = attr_loc_obj.get("loc_end") {
//                     //                 //         // Check if loc_end is an object
//                     //                 //         if let Value::Object(loc_end_obj) = loc_end {
//                     //                 //             // Access the pos_bol, pos_cnum, pos_fname, pos_lnum values
//                     //                 //             println!("pos_bol: {}", loc_end_obj.get("pos_bol").unwrap());
//                     //                 //             println!("pos_cnum: {}", loc_end_obj.get("pos_cnum").unwrap());
//                     //                 //             println!("pos_fname: {}", loc_end_obj.get("pos_fname").unwrap());
//                     //                 //             println!("pos_lnum: {}", loc_end_obj.get("pos_lnum").unwrap());
//                     //                 //         }
//                     //                 //     }
//                     //                 // }
//                     //             }
//                     //         }
//                         // }
// //                }
//                 }
//             }
//         }
//     }

    // println!("The value is: {}", json.as_str().unwrap());
    // return json
    // let hash_algorithms = ["SHA1", "MD5", "CRC32"];
    // let model = SentenceEmbeddingsModel::new(ModelType::HuggingFaceTransformers, model_path).unwrap();

    // let mut simple_hashing_collisions = Vec::new();
    // let mut fuzzy_matching_collisions = Vec::new();
    // let mut nn_vectorization_collisions = Vec::new();

     // for (key, value) in json.as_object().unwrap().iter() {
     // 	 //     let key_hashes: Vec<String> = hash_algorithms.iter().map(|algorithm| hash_string(algorithm, key)).collect();
     // 	 for (key2, value2) in json.as_object().unwrap().iter() {
     // 	     println!("{}{}{}{}",key,key2,value,value2);	     
     // 	 }
     // }
    
    //         if key != key2 {
    //             let key2_hashes: Vec<String> = hash_algorithms.iter().map(|algorithm| hash_string(algorithm, key2)).collect();

    //             if key_hashes.iter().all(|hash| key2_hashes.contains(hash)) {
    //                 simple_hashing_collisions.push(json!({"key1": key, "value1": value, "key2": key2, "value2": value2}));
    //             }

    //             let distance = levenshtein_distance(key, key2);
    //             if distance <= 2 {
    //                 fuzzy_matching_collisions.push(json!({"key1": key, "value1": value, "key2": key2, "value2": value2, "distance": distance}));
    //             }

    //             let distance = word2vec_distance(&model, key, key2);
    //             if distance >= 0.5 {
    //                 nn_vectorization_collisions.push(json!({"key1": key, "value1": value, "key2": key2, "value2": value2, "distance": distance}));
    //             }
    //         }
    //     }
    // }

     let output_json = json!({
         "simple_hashing": {}});
    // // 	    let output_json = json!({
    // // 		"simple_hashing": {"collisions": simple_hashing_collisions},
    // // 		"fuzzy_matching": {"collisions": fuzzy_matching_collisions},
    // // 		"nn_vectorization": {"collisions": nn_vectorization_collisions}
    // // 	    });
    // 	}
    // });
    return output_json
}



fn main() {
    for argument in env::args_os().skip(1) {
	let json_file_path = argument;

	let s = json_file_path.into_string().unwrap();
//	println!("DEBG {:?}",s);
	let output_json = self_similar_search(&s);
//	println!("{}",output_json.to_string());
    }
}

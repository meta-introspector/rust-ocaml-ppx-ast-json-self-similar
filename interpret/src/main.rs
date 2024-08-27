use std::env;
use std::fs::File;

// interprets a stack of json
// pass in a bunch of json files and it will apply the second one to the
// first. the application of the third to the second to the first will be applied.
// each time it will apply the new file to the current stack or interpret that stack
// that lets it rewrite itself. but it can only append the new data to the end so its results are appended
// that way no data is lost, but it might get shadowed.
    
type Stack = Vec<serde_json::Value>;

fn interpret(s:&Stack)->Result<serde_json::Value,()> {
    //Ok(serde_json::Value::Array(s.to_vec()))
    Ok(serde_json::Number::from(s.to_vec().len()).into())
}
fn main() {
    //println!("Interprets a !");

    let mut stack : Stack = Stack::new();
    
    for argument in env::args_os().skip(1) {
        let json_file_path = argument;
        let s = json_file_path.into_string().unwrap();
        //println!("read = {:?}", s);
        let deserialized: serde_json::Value = serde_json::from_reader(File::open(s).unwrap()).unwrap();
        //println!("deserialized = {:?}", deserialized);
	// now lets construct a stack of these
	stack.push(deserialized);	
	stack.push(interpret(&stack).unwrap())
	    
    }
    let s = serde_json::to_string(&stack);
    println!("{}", s.unwrap());// this should have the results at the top of the stack
}


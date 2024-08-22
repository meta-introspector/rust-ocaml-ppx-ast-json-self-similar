pub fn end_array()->bool{ return false;}
pub fn end_object()->bool{ return false;}
pub fn find_object(depth:usize,
		   inname:&str,
		   parts0: &str,
		   parts1: &str,
		   outname: &str,
		   _children: &Vec<bool>)->bool{
    println!("find_object({},\"{}\",\"{}\",\"{}\",\"{}\"",
	     depth,
	     inname,
	     parts0,
	     parts1,
	     outname);
    return false
}
pub fn find_array(depth:usize,
	      i:usize,
	      inname: &str,
		  varname_item: &str,
		  _children: &Vec<bool>)->bool{
    println!("find_array({},{},\"{}\",\"{}\"",
	     depth,
	     i,
	     inname,
	     varname_item);

    return false
}

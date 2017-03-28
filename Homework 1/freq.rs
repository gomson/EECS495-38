use std::collections::HashMap;
use std::fs::File;
use std::io::{Read,BufReader};
fn main(){
	let mut input = String::new();
	let f = File::open("./test.txt").unwrap();
	let mut br = BufReader::new(f);
	br.read_to_string(&mut input).unwrap();
	let v: Vec<&str> = input.split(|c: char| !c.is_alphanumeric()).collect();
	let mut hashmap: HashMap<&str, usize> = HashMap::new();
	for i in &v{
		if !i.is_empty(){
			if !hashmap.contains_key(i){
				hashmap.insert(i, 1);
			}
			else{
				*hashmap.get_mut(i).unwrap() += 1;
			}
		}
	}
	let mut output = Vec::new();
	for (key, val) in &hashmap{
		output.push((key, val));
	}
	output.sort_by(|&a, &b| b.1.cmp(a.1));
	for i in &output{
		println!("{}: {}", i.0, i.1);
	}
}
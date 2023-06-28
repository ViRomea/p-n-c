mod db {
use std::collections::HashMap;
use std::fs;
const stg: String = String::new();

fn init(p:i64, a:String) -> Session {
	let it = &Session{
		passwd: &p, 
		name: &String::from(a),
		storage: &String,
	};
	it
}

fn set_stg(stg_name: String) {
	stg = String::from("{stg_name}");
}

fn out(key: &String, mean) {
	
	
	
	
}

fn change() {
	
	
	
}


fn delete() {
	
	
}

struct Session { 
	passwd: i64,
	name:   String,
	storage: String
	content:Vec<HashMap<String, i64>>,
}

} 



















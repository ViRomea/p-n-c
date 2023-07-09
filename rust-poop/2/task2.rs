fn main() {
	println!("{}", reverse("Godot"));
}

fn reverse(input: &str) -> String {
	let mut first_vec = Vec::new();
	
	for n in input.chars() {
		first_vec.push(n);
	};

	let mut second_vec: Vec<char> = Vec::new();
	
	let fvl = first_vec.len();
	let svl = second_vec.len();
	
	let mut i = 0;
	while i < fvl {
		let mut n = first_vec.len() - i;
		let mut l = first_vec.get(n);
		let mut a = l.to_string();
		println!("{}", p.to_string());
		i+=1;
	};
	let output = second_vec.into_iter().collect();
	output
}

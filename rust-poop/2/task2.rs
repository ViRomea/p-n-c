fn main() {
	reverse("Godot");
}

fn reverse(input: &str) -> String {
	let mut first_vec = Vec::new();
	for n in input.chars() {
		first_vec.push(n);
	};
	let mut second_vec: Vec<char> = Vec::new();
	let fvl = first_vec.len();
	let svl = second_vec.len();
	let mut i = 1;
	while i < fvl+1 {
		let mut n = first_vec[fvl-i];
		second_vec.push(n);
		i += 1;
		println!("{:?}", second_vec);
	};
	let a: String = second_vec.into_iter().collect::<String>();
	println!("{}", a);
	a
}

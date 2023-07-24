use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
	let mut hs = HashSet::new();
	let a = sort_string(word.to_lower_case());
	let mut q = 0;
	for n in possible_anagrams {
		if n != a {
			for b in a {
				if b in n {
					q += 1;
				};
				
				if q == a.len() {
					hs.insert(n);
				};
			};
		};
	};
	hs
}


fn sort_string(s: String) -> Vec<chars> {
	let mut chars: Vec<chars> = s.chars().collect().sort_unstable();
	chars
}

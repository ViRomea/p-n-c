use std::fmt;

fn main() {
	let mut t = Triangle::new(5.0, 3.2, 6.7, TrKinds::Usual);
	println!("{}", t);
}

#[derive(PartialEq)]
enum TrKinds {
	Usual,
	
	Rectangular,
	Isosceles,
	Both,
	
	Equilateral,
}

struct Triangle {
	a:f32,
	b:f32,
	c:f32,
	kind:TrKinds,
}

trait Calcs {
	fn get_area(&self) -> f32;
}

trait Generate<T> {
	fn new(a: f32, b: f32, c: f32, k: T) -> Self;
}

impl Generate<TrKinds> for Triangle {
	fn new(a: f32, b: f32, c: f32, k: TrKinds,) -> Triangle {
		
		Triangle {
			a: a,
			b: b,
			c: c,
			kind: k,
		}
	}
}

impl Calcs for Triangle {
	fn get_area(&self) -> f32 {
		
		if self.kind == TrKinds::Rectangular { 
			let area = self.a * self.b * 0.5;
			return area;
		} else if self.kind == TrKinds::Usual {
			let p = (self.a + self.b + self.c) / 2.0;
			let area2 = (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt();
			return area2;
			
		} else {
			return 0.0;
		};
	}
}

impl fmt::Display for Triangle {
	fn fmt(&self,f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let s1 = r"     /\     ";
		let s2 = r"    /  \    "; 
		let s3 = r"   /____\   ";
		
		let r = format!("{}\n{}\n{}" ,s1, s2, s3);
		write!(f, "{}", r)
		
	}
}



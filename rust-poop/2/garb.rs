use std::io;

fn main() {
	let mut pig = Liver::<Mobs>{
		kind: Mobs::Enemy, 
		hp:100, 
		weap1:Weapon{ damage:20}
	};
	
	let YOU = Liver::<Mobs>{
		kind: Mobs::Player,
		hp:200, 
		weap1:Weapon{ damage:30}
	};
	
	damage(&mut pig, &YOU);
}

fn damage<T>(direction: &mut Liver<T>, source: &Liver<T>) {
	direction.hp -= source.weap1.damage;
	println!("{}", direction.hp);
}

fn start_game() {
	println!("1 - random fight\n 2 - close game");
	let mut s = &String::new();
	io::stdin().read_line(&mut s);
	
	
	match s {
		 => start_fight(),
		 => println!("Game is closed")
	};
}

fn start_fight() {
	 
}

enum Mobs {
	Player,
	Enemy,
}

struct Liver<T> {
	kind: T,
	hp: i32,
	weap1: Weapon,
}

struct Rat {
	
}

struct Weapon {
	damage: i32,
}




pub mod Begin
{
	use std::io;


	pub fn start_game() -> Entities
	{

		let b: bool = true;

		while b == true
		{
			let mut run: String = String::new();
			io::stdin().read_line(&mut run);

			let play: String = String::from("play");


			return match run
			{
				play => Entities::make_player(),
			}
		};


		let mut name: String = String::new();
		io::stdin().read_line(&mut name);

		let mut PERSON = Entities::Player(SPlayer {name: name, health: [100, 100], cords: [0, 0] });
		PERSON
	}


	pub struct Game
	{



	}



	pub struct SPlayer
	{
		name: String,
		health: [usize; 2],
		cords: [usize; 2],
	}

	pub struct Mob 
	{
		health: [usize; 2],
		cords: [usize; 2],
	}

	pub enum Entities
	{
		Player(SPlayer),
		Abadeon(Mob),
		Dog(Mob),
		Rat(Mob)
	}

	impl Entities
	{
		fn make_player() -> Entities {
			let mut name: String = String::new();
			io::stdin().read_line(&mut name);

			let you: Entities = Entities::Player(SPlayer { name: name, health: [100, 100], cords: [0, 0]});
			you


		}

	}

	fn fight(&Entities) {




	}

use crate::game::utils::question;

pub struct Game {
	pub player: Player,
	pub scenes: Vec<Scene>,
	pub debug: bool,
}

pub type Scene = fn(game: &Game);

pub struct Player {
	pub name: String,
	pub health: f32,
	pub coins: i32,
	pub weapons: [Box<Weapon>; 2]
	// items: Vec<Item>
}

impl Player {
	pub fn init() -> Self {
		// TODO: Add initial weapon selection
		Player {
			name: question("Please select a name for yourself", &None),
			health: 100.0,
			coins: 0,
			weapons: [Box::new(BlankWeapon{}), Box::new(BlankWeapon{})],
			// items: Vec::new()
		}
	}
}

struct BlankWeapon {}

impl Weapon for BlankWeapon {
	fn name(&self) -> String {
		String::from("")
	}

	fn description(&self) -> String {
		String::from("")
	}

	fn attack(&self, player: &mut Player, enemy: &mut Enemy) {}
}

pub trait Weapon {
	fn name(&self) -> String;
	fn description(&self) -> String;
	fn attack(&self, player: &mut Player, enemy: &mut Enemy);
}

pub struct Enemy {
	pub name: String,
	pub health: f32
}
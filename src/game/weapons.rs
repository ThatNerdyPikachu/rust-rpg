use crate::game::structs::*;

pub struct Flashlight{}

impl Weapon for Flashlight {
	fn name(&self) -> String {
		String::from("Communist Flashlight")
	}

	fn description(&self) -> String {
		String::from("A covert metal flashlight.\nHighly effective against anti-Communists.")
	}

	fn attack(&self, player: &mut Player, enemy: &mut Enemy) {
		// TODO: Actually do this
	}
}

pub struct Broom{}

impl Weapon for Broom {
	fn name(&self) -> String {
		String::from("Anti-Communist Broom")
	}

	fn description(&self) -> String {
		String::from("A wooden broom, which looks extremely epic.\nHighly effective against Communists.")
	}

	fn attack(&self, player: &mut Player, enemy: &mut Enemy) {
		// TODO: Actually do this
	}
}
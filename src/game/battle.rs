use crate::game::structs::*;

pub enum BattleResult {
	Successful,
	Defeated
}

enum BattleMenuState {
	Top,
	Attack
	// Items
}

impl Game {
	pub fn battle(&self, enemy: &mut Enemy) -> BattleResult {
		// TODO: Actually do this

		BattleResult::Successful
	}
}
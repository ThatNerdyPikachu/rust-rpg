use crate::game::structs::*;
use crate::game::utils::*;

pub fn get_scenes() -> Vec<Scene> {
	vec![a_dark_room]
}

pub fn a_dark_room(game: &Game) {
	println!("Cool, a scene!");
	if game.debug {
		println!("Cool, debug~");
	}

	print(format!("You see... nothing.\n{}",
		"You are able to make out a few shapes, like a square, but that's it.")
	.as_str());
	wait();
}
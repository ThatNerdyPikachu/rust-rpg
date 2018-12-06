#![allow(dead_code)]
#![allow(unused_variables)]

mod structs;
mod utils;
mod scenes;
mod weapons;
mod battle;

pub fn start(debug: bool) {
	// TODO: save(s).json loading
	let game = structs::Game{
		player: structs::Player::init(),
		scenes: scenes::get_scenes(),
		debug
	};

	game.scenes[0](&game);
}
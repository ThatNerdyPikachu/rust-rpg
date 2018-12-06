use std::{io, time, thread};
use std::io::Write;

pub fn question(prompt: &str, valid: &Option<&[&str]>) -> String {
	loop {
		print!("{}", prompt);

		if valid.is_some() {
			print!(" ({}): ", valid.as_ref().unwrap().join(", "));
		} else {
			print!(": ");
		}

		io::stdout().flush().expect("Failed to flush stdout!");

		let mut response = String::new();

		io::stdin().read_line(&mut response).expect("Failed to read from stdin!");

		response = String::from(response.trim());

		if valid.is_some() {
			if valid.as_ref().unwrap().contains(&response.as_str()) {
				return response;
			} else {
				println!("\"{}\" is not a valid answer!", response);
			}
		} else if response == "" {
			println!("Your answer cannot be blank!");
		} else {
			return response;
		}
	}
}

pub fn print(text: &str) {
	for s in text.split("") {
		print!("{}", s);
		thread::sleep(time::Duration::from_millis(30));
		io::stdout().flush().expect("Failed to flush stdout!");
	}
}

pub fn wait() {
	io::stdin().read_line(&mut String::new()).expect("Failed to read from stdin!");
}

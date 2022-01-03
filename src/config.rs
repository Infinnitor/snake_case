use std::env::args;

pub const WIDTH: i32 = 35;
pub const HEIGHT: i32 = 20;

pub const FRAME_WAIT_MILLIS: u32 = 85;
pub const KEYBOARD_UPDATES_PER_FRAME: u32 = 5;

pub const COLOUR_CODES_SUPPORTED: bool = true;

pub fn flagged(names: Vec<&str>) -> bool {
	let argv: Vec<String> = args().collect();

	for n in names {
		if argv.contains(&String::from(n)) {
			return true;
		}
	}

	return false;
}

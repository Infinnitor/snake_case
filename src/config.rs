use std::env::args;

pub const WIDTH: i32 = 35;
pub const HEIGHT: i32 = 20;

pub const FRAME_WAIT_MILLIS: u64 = 85;

pub const OS: &str = "UNIX";

pub fn flagged(names: Vec<&str>) -> bool {
	let argv: Vec<String> = args().collect();

	for n in names {
		if argv.contains(&String::from(n)) {
			return true;
		}
	}

	return false;
}
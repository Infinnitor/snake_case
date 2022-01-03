use super::config;


pub fn cursorup(a: &i32) -> String {
	if config::OS == "UNIX" {
		String::from(format!("\x1b[{}A\x1b[0G", a.to_string()))
	}

	// Windows uses different cursormove escape sequences
	else {
		String::from(format!("\x1b[0;{}f", a.to_string()))
	}

}


pub fn clear_console() {
	if config::OS == "UNIX" {
		print!("\x1bc");
	}

	else {
		print!("\x1b[2J");
	}
}


// Colour escape sequences differ between Ẇindows and Unix
// They also differ between different terminal emulators
// Not sure what to do about checking whether different terminal emulators support it lol
pub fn block_rgb(r: u32, g: u32, b: u32) -> String {
	if !config::COLOUR_CODES_SUPPORTED {
		panic!("OS does not support ANSI colour codes");
	}
	String::from(format!("\x1b[48;2;{};{};{}m", &r.to_string(), &g.to_string(), &b.to_string()))
}


pub fn single_block_rgb(r: u32, g: u32, b: u32) -> String {
	if !config::COLOUR_CODES_SUPPORTED {
		panic!("OS does not support ANSI colour codes");
	}
	let b: String = String::from(format!("\x1b[48;2;{};{};{}m", &r.to_string(), &g.to_string(), &b.to_string()));
	return format!("{} {}", b, block_rgb(0, 0, 0));
}


pub fn fore_rgb(r: u32, g: u32, b: u32) -> String {
	if !config::COLOUR_CODES_SUPPORTED {
		panic!("OS does not support ANSI colour codes");
	}
	String::from(format!("\x1b[38;2;{};{};{}m", &r.to_string(), &g.to_string(), &b.to_string()))
}

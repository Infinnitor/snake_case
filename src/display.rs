use super::escapes;
use super::config;
use std::process::exit;


#[derive(Copy, Clone)]
pub struct rgbc {
	r: u16,
	g: u16,
	b: u16
}


impl rgbc {
	pub fn to_string(&self) -> String {
		String::from(format!("rgb({}, {}, {})", self.r, self.g, self.b))
	}

	pub fn check_colour_code_support() {
		if !config::COLOUR_CODES_SUPPORTED {
			println!("Your operating system doesn't support ANSI colour code escape sequences. Avoid using --colour or --greyscale options");
			exit(0);
		}
	}

	pub fn to_escape_sequence(&self) -> String {
		if config::flagged(vec!["-C", "--colour", "--color"]) {
			rgbc::check_colour_code_support();
			return escapes::single_block_rgb(self.r as u32, self.g as u32, self.b as u32);
		}

		else if config::flagged(vec!["-g", "--greyscale", "--grayscale"]) {
			rgbc::check_colour_code_support();

			let g: u32 = (self.total() / 3) as u32;
			return escapes::single_block_rgb(g, g, g);
		}

		// shit code, should've just hardcoded colours
		else {
			if self.total() < 155 {
				return String::from(" ");
			}

			if self.r > self.g && self.r > self.b {
				return String::from("o");
			}

			if self.g > self.r && self.g > self.b {
				return String::from("y");
			}

			if self.b > self.r && self.b > self.g {
				return String::from("-");
			}

			return String::from(" ");
		}
	}

	pub fn total(&self) -> i32 {
		(self.r + self.g + self.b) as i32
	}
}


pub fn RGB(red: u16, green: u16, blue: u16) -> rgbc {
	rgbc {
		r: red, g: green, b: blue
	}
}


pub struct surface {
	w: i32,
	h: i32,

	pixels: Vec<rgbc>
}


impl surface {
	pub fn new(width: i32, height: i32) -> Self {
		let mut px_vec = vec![];
		for p in 0..width*height {
			px_vec.push(RGB(0, 0, 0));
		}

		surface {
			w: width,
			h: height,
			pixels: px_vec
		}
	}

	pub fn get(&self, x: i32, y: i32) -> rgbc {
		self.pixels[(y*self.w + x) as usize]
	}

	pub fn set(&mut self, x: i32, y: i32, c: rgbc) {
		self.pixels[(y*self.w + x) as usize] = c;
	}

	pub fn fill(&mut self, c: rgbc) {
		for y in 0..self.h {
			for x in 0..self.w {
				self.set(x, y, c);
			}
		}
	}

	pub fn rect(&mut self, rx: i32, ry: i32, rw: i32, rh: i32, c: rgbc) {
		for y in 0..rh {
			for x in 0..rw {
				self.set(rx+x, ry+y, c);
			}
		}
	}

	pub fn clear(&self) {
		escapes::clear_console();
	}

	pub fn flip(&self) {
		let mut display_screen: String = String::new();

		for y in 0..self.h {
			for x in 0..self.w {
				display_screen += &self.get(x, y).to_escape_sequence();
				display_screen += &self.get(x, y).to_escape_sequence();
			}
			display_screen += "\n";
		}

		print!("{}", display_screen);
	}

	pub fn width(&self) -> i32 {
		self.w
	}

	pub fn height(&self) -> i32 {
		self.h
	}
}

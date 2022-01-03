use std::process::exit;
use std::time::{Instant};

use device_query::{Keycode};
use super::display::{rgbc, RGB, surface};

use super::escapes;
use super::util;

use super::config::{WIDTH, HEIGHT};

const PLAYER_START_LENGTH: i32 = 5;
const PLAYER_GROW_BY: i32 = 3;


pub struct player {
	x: i32,
	y: i32,
	length: i32,

	xvel: i32,
	yvel: i32,

	snakebits: Vec<snakebit>,
	yummle: yummy_thing,

	spawn_time: Instant,
}


impl player {
	pub fn new(start_x: i32, start_y: i32) -> Self {
		player {
			x: start_x,
			y: start_y,
			length: PLAYER_START_LENGTH,

			xvel: 1, yvel: 0,

			snakebits: vec![],
			yummle: yummy_thing::rand(), // Default start at 0, 0

			spawn_time: Instant::now()
		}
	}

	pub fn setvel(&mut self, xv: i32, yv: i32) {
		self.xvel = xv;
		self.yvel = yv;
	}

	fn spawn_yummle(&mut self) {
		self.yummle = yummy_thing::rand();
		if self.collide_yummle() {
			self.spawn_yummle();
		}

	}

	fn headcollide_yummle(&self) -> bool {
		return (self.x == self.yummle.x) && (self.y == self.yummle.y);
	}

	fn collide_yummle(&self) -> bool {
		if self.headcollide_yummle() { return true; }

		for bit in &self.snakebits {
			if bit.x == self.yummle.y && bit.y == self.yummle.y {
				return true;
			}
		}

		return false;
	}

	fn collide_self(&self) -> bool {
		for bit in &self.snakebits {
			if bit.lifetime > self.length - (3+PLAYER_GROW_BY) {
				continue;
			}

			if bit.x == self.x && bit.y == self.y {
				return true;
			}
		}

		return false;
	}

	pub fn update(&mut self, main_window: &mut surface, keylist: &mut Vec<Keycode>, frames: &u64) {
		// Cannot use match in a for loop because match doesn't recognise Keycode
		if (keylist.contains(&Keycode::A) || keylist.contains(&Keycode::Left)) && self.xvel != 1 {
			self.setvel(-1, 0);
		}

		else if (keylist.contains(&Keycode::D) || keylist.contains(&Keycode::Right)) && self.xvel != -1 {
			self.setvel(1, 0);
		}

		else if (keylist.contains(&Keycode::W) || keylist.contains(&Keycode::Up)) && self.yvel != 1 {
			self.setvel(0, -1);
		}

		else if (keylist.contains(&Keycode::S) || keylist.contains(&Keycode::Down)) && self.yvel != -1 {
			self.setvel(0, 1);
		}

		if !keylist.is_empty() {
			super::escapes::clear_console();
		}

		self.x += self.xvel;
		self.y += self.yvel;

		main_window.set(self.x, self.y, RGB(155, 35, 35));
		// main_window.set(self.x+1, self.y, RGB(155, 35, 35));

		self.snakebits.push(snakebit::new(self.x, self.y, self.length));

		for i in (0..self.snakebits.len()-1).rev() {
			let mut bit: &mut snakebit = &mut self.snakebits[i];

			bit.update(main_window);
			if bit.lifegone {
				self.snakebits.remove(i);
			}
		}

		self.yummle.update(main_window);
		if self.headcollide_yummle() {
			self.spawn_yummle();
			self.length += PLAYER_GROW_BY;
		}

		// Check for gameover
		if self.collide_self() {
			self.gameover(main_window);
		}

		if self.x > WIDTH-1 || self.x < 1 {
			self.gameover(main_window);
		}

		if self.y < 1 || self.y == HEIGHT-1 {
			self.gameover(main_window);
		}
	}

	pub fn gameover(&self, main_window: &mut surface) {
		escapes::clear_console();
		draw_border(main_window);
		main_window.flip();
		println!("FINAL LENGTH : {}", self.length);
		println!("TIME ALIVE : {} secs", self.spawn_time.elapsed().as_secs());
		exit(0);
	}
}


struct snakebit {
	x: i32,
	y: i32,
	lifetime: i32,
	lifegone: bool
}


impl snakebit {
	pub fn new(start_x: i32, start_y: i32, life: i32) -> Self {

		snakebit {
			x: start_x,
			y: start_y,
			lifetime: life,
			lifegone: false
		}
	}

	pub fn update(&mut self, main_window: &mut surface) {
		self.lifetime -= 1;
		if self.lifetime < 1 {
			self.lifegone = true;
		}

		main_window.set(self.x, self.y, RGB(155, 35, 35));
	}
}


pub struct yummy_thing {
	pub x: i32,
	pub y: i32,
}


impl yummy_thing {
	pub fn new(start_x: i32, start_y: i32) -> Self {
		yummy_thing {
			x: start_x,
			y: start_y
		}
	}

	pub fn rand() -> Self {
		yummy_thing {
			x: util::randint(3, WIDTH - 3),
			y: util::randint(3, HEIGHT - 3)
		}
	}

	pub fn update(&self, main_window: &mut surface) {
		main_window.set(self.x, self.y, RGB(35, 155, 35));
	}
}


pub fn draw_border(main_window: &mut surface) {
	main_window.rect(0, 0, 1, main_window.height(), RGB(35, 35, 135));
	main_window.rect(main_window.width()-1, 0, 1, main_window.height(), RGB(35, 35, 135));

	main_window.rect(0, 0, main_window.width(), 1, RGB(35, 35, 135));
	main_window.rect(0, main_window.height()-1, main_window.width(), 1, RGB(35, 35, 135));
}

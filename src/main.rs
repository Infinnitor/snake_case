use std::thread::sleep;
use std::time::{Duration};

#[allow(dead_code, non_camel_case_types)]
mod gameobjects;
#[allow(dead_code, non_camel_case_types)]
mod escapes;

#[allow(dead_code, non_camel_case_types)]
mod display;
use display::{rgbc, RGB, surface};

#[allow(dead_code, non_camel_case_types)]
mod util;

mod config;
use config::{WIDTH, HEIGHT, FRAME_WAIT_MILLIS};
pub const DESIRED_FRAMERATE: Duration = Duration::from_millis(FRAME_WAIT_MILLIS);


fn main() {
	mainloop();
}


fn mainloop() {
	let BACKGROUND: rgbc = RGB(10, 9, 13);

	let mut player_inst: gameobjects::player = gameobjects::player::new(5, 10);
	let mut main_window: surface = display::surface::new(WIDTH, HEIGHT);

	escapes::clear_console();

	let mut frames: u64 = 0;
	loop {
		main_window.fill(BACKGROUND);

		player_inst.update(&mut main_window, &frames);
		gameobjects::draw_border(&mut main_window);

		main_window.flip();
		print!("{}", escapes::cursorup(&HEIGHT));

		frames += 1;
		sleep(DESIRED_FRAMERATE);
	}
}

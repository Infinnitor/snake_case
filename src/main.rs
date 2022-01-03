use std::thread::sleep;
use std::time::{Duration};
use device_query::{DeviceQuery, DeviceState, Keycode}; // the goat

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
use config::{WIDTH, HEIGHT, FRAME_WAIT_MILLIS, KEYBOARD_UPDATES_PER_FRAME};
pub const TICK_TIME: Duration = Duration::from_millis((FRAME_WAIT_MILLIS / KEYBOARD_UPDATES_PER_FRAME) as u64);


fn main() {
	let BACKGROUND: rgbc = RGB(10, 9, 13);

	let mut player_inst: gameobjects::player = gameobjects::player::new(5, 10);
	let mut main_window: surface = display::surface::new(WIDTH, HEIGHT);

	let keyboard = DeviceState::new();
	let mut keylist: Vec<Keycode> = vec![];

	let mut frames: u64 = 0;
	loop {
		main_window.fill(BACKGROUND);

		player_inst.update(&mut main_window, &mut keylist, &frames);
		gameobjects::draw_border(&mut main_window);

		main_window.flip();
		print!("{}", escapes::cursorup(&HEIGHT));

		frames += 1;
		for x in 0..KEYBOARD_UPDATES_PER_FRAME {
			keylist = keyboard.get_keys();
			sleep(TICK_TIME);
		}
		escapes::clear_console();
	}
}

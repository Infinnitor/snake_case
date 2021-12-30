use rand::Rng;

pub fn randint(a: i32, b: i32) -> i32 {
	let mut rng = rand::thread_rng();
	return rng.gen_range(a..b);
}

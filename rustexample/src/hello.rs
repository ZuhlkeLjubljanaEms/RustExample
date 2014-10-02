extern crate time;

pub fn hello() {
	let time_now = time::now();
	println!("Hello World! The time is {}", time::strftime("%a, %d %b %Y %T %z", &time_now));
}

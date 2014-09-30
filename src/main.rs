#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use vectors::Trait;

pub mod hello;
pub mod logging;
pub mod vectors;

fn main() {
	println!("Executing function hello()");
	hello::hello();
	println!("Executing function logging()");
	logging::logging();
	println!("Executing function vectors()");
	vectors::vectors();
	println!("Done");
}

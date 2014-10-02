#![feature(phase)]
#[phase(plugin, link)] extern crate log;

fn main() {
	println!("Hello from submodule1");
}

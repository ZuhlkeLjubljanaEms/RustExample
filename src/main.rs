mod hello;
mod vectors;

fn main() {
	println!("Executing function hello()");
	hello::hello();
	println!("Executing function vectors()");
	vectors::vectors();
	println!("Done");
}


fn main() {
	let s: String = String::from("Hello, Rust!");
	let s_again: String = take_and_return(s); 
	println!("{}", s_again);
}

fn take_and_return(s: String) -> String {
	s
}


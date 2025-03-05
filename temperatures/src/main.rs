use std::io;

fn main() {
	println!("Convert to fahrenheit");
	
	loop {
		println!("Provide celsius to convert: ");
		let mut celsius = String::new();
		io::stdin()
			.read_line(&mut celsius)
			.expect("Failed to read line");
	
		if celsius.trim() == "exit" {
			break println!("Exiting...");
		}
	
		let celsius: u32 = match celsius.trim().parse() {
			Ok(num) => num, 
			Err(_) => continue,
		};
		
		
		let fahrenheit: u32 = (celsius * 9/5) + 32;
		
		println!("{celsius} celsius is {fahrenheit} fahrenheit");
		
	}
	
	
	
}

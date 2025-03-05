use std::io;

fn main() {
	// print n fibonacci numbers
	
	loop {
		println!("Provide n: ");
		
		let mut n = String::new();
		
		io::stdin()
			.read_line(&mut n)
			.expect("Failed to read line");
		
		let n = n.trim();
		
		if n == "exit" {
			break println!("Exitting...");
		};
		
		let n: u32 = match n.parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		
		let mut s: u32 = 1;
		let mut f: u32 = 1;
		

		println!("Printing fibonacci numbers");
		for i in 0..n {
			if i <= 1 {
				println!("{i}");
			} else if i == 2 {
				println!("1");
			} else {
				let t = s + f;
				println!("{t}");
				s = f;
				f = t;
			}
		}
	
	
	
	}
	
	
}

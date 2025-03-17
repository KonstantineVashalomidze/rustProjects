fn main() {
	/* If we don't explicitely assign type to a variable, then compiler will infer one for us. */
	let x: i32 = 5;
	let mut y = 5;
	
	y = x;
	
	let z = 10;
	
	println!("Success!");
	
	
	let v: u16 = 255_u8 as u16;
	
	println!("Success!");
	
}

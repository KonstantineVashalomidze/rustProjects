
fn main() {
	let s = String::from("Hello");
	
	// these are equal
	let slice1 = &s[0..1]; 
	let slice2 = &s[..2];
	
	println!("{} {}", slice1, slice2);
	
	let len = s.len();
	
	// these are equal too
	let slice1 = &s[3..len];
	let slice2 = &s[3..];
	
	println!("{} {}", slice1, slice2);
	
	
	// these are also equal
	let slice1 = &s[0..len];
	let slice2 = &s[..];
	
	println!("{} {}", slice1, slice2);
	
	
	
}



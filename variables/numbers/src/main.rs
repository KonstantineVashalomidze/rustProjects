fn main() {
	// If we don't explicitely assign a type to a variable, then the compiler 
	// will infer one for us.
	let x: i32 = 5;
	let mut y: u32 = 5;
	
	
	let z = 10;
	println!("Success!");

	/************************************************/

	let v: u16 = 38_u8 as u16;
	
	println!("Success!");
	
	/***********************************************/
	let x: u32 = 5;
	assert_eq!("u32".to_string(), type_of(&x));
	
	println!("Success!");

}

fn type_of<T>(_: &T) -> String {
	format!("{}", std::any::type_name::<T>());
}





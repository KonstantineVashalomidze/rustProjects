fn main() {
	/* Binding and mutability */
	// A variable can be used only if it has been initialised
	let x: i32 = 5; 
	let y: i32;
	
	assert_eq!(x, 5);
	println!("Success!");
	/***********************************/
	
	// Mark x as mutable
	let mut x = 1; 
	x += 2;
	
	assert_eq!(x, 3);
	println!("Success!");
	
	/* Scope */
	// A scope is a range within the program for which the item is valid.
	let x: i32 = 10;
	{
		let y: i32 = 5;
		println!("The value of x is {} and value of y is {}", x, y);
	}
	
	println!("The value of x is {}", x);
	
	/* Shadowing */
	let x: i32 = 5;
	{
		let x = 12;
		assert_eq!(x, 12);
	}
	
	assert_eq!(x, 5);
	
	let x = 42;
	println!("{}", x); // Prints "42".
	
	
	
	let mut x: i32 = 1;
	x = 7;
	
	// Shadowing and re-binding
	let x = x;
	
	
	let y = 4;
	// Shadowing
	let y = "I can also be bound to text!";
	
	println!("Success!");
	
	
	/* Unused variables */
	let x = 1;
	println!("{x}");
	
	/* Destructuring tuples */
	let (mut x, y) = (1, 2);
	x += 2;
	
	assert_eq!(x, 3);
	assert_eq!(y, 2);
	
	println!("Success!");
	
	
	/* Destructuring assignments */
 	let (x, y);
 	(x,..) = (3, 4); // Obtain tuple head
 	[.., y] = [1, 2]; // Obtain array tail
 	assert_eq!([x,y], [3, 2]); 
 	println!("Success!");
 	
 	
 	
 	
	
}

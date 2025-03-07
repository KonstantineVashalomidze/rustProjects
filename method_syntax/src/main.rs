#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

// this implements Rectangle struct 
impl Rectangle {

	/* area, wdth, and can_hold are called associated functions since they are associated with Rectangle type */
	// takes reference of rectangle instance
	// here &self is same as writing self: &Self
	fn area(&self) -> u32 { // we can define methods on structs itself
		self.width * self.height
	}
	// if we wanted to have mutator method we should have passed &mut slef as parameter
	// direct self is also possible but method takes ownership of the instance
	fn width(&self) -> bool { // we can have method with same name as attribute of struct
		self.width > 0
	}
	
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
	
	/* We can also define associated functions that doesn't have self parameter
	   because they don't need the instance of the type to work with. Think about 
	   them as static methods of java. associated fucntions that doesn't have self 
	   parameter are not methods. these are often used to construct instance of the 
	   struct 
	 */
	// to construct instance of the Rectangle:
	fn new(width: u32, height: u32) -> Self { // note self is type as other types
		Self { // struct is also expression so it can be directly returned
			width, 
			height,
		}
	}
	
}
/* we can have multiple implementations of same struct */
impl Rectangle {
	fn print(&self) {
		println!("Rectangle with width={} and height={}", self.width, self.height);
	}
}


fn main() {
	let rect1 = Rectangle {
		width: 30, 
		height: 50,
	};
	
	let rect2 = Rectangle {
		width: 20,
		height: 40,
	};
	
	println!(
		"The area of the rectangle is {} square pixels.",
		rect1.area() // we can directly call area method on struct
	);
	
	println!("{}", rect1.width());
	
	// here rust automatically determines that rect1 should be reference and I 
	// don't have to manually type & on rect1
	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); 
	
	let rect3 = Rectangle::new(29, 14);
	println!("Can rect3 {:?}", rect3);
	
	rect3.print();
	
	
}





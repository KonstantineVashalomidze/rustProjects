struct User {
	active: bool,
	username: String, 
	email: String, 
	sign_in_count: u64,
} // note this doesn't require semicolon



struct Color(i32, i32, i32); // note this require semicolon

struct Pair(i32, i32);


fn builder_user(email: String, username: String) -> User 
{
	 User {
		active: true,
		username: username,
		email,
		sign_in_count: 1,
	 }
}


fn main() {
	let mut concrete_user = builder_user(
										String::from("k_vashalomidze@mail.ru"),
										String::from("Mama africa")
									);
									
	println!("{}", concrete_user.active);
	concrete_user.active = false;
	println!("{}", concrete_user.active);
	
/*	let _concrete_user2 = User { // this is bad to do
		active: concrete_user.active,
		username: concrete_user.username,
		email: String::from("vashalomidzekonstantine@gmail.com"),
		sign_in_count: concrete_user.sign_in_count,
	};
	*/
	
	
	// this is better inherits everything and updtes email from concrete_user
	let concrete_user3 = User {
		email: String::from("vashalomidzekonstantine@gmail.com"),
		..concrete_user // note we don't use comma here
	};
	
	//println!("{}", concrete_user.username); this is invalid since user3 borrowed username from concrete user
	
	
	let black = Color(0, 0, 0);
	let pair = Pair(2, 3);
	
	println!("{}", pair.1); // access elements like this
	
	// we could also destructure struct tuples like this:
	let Color(r, b, g) = black;
	println!("{} {} {}", r, g, b);
	
	
}












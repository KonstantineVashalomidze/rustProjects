fn main() {
	let s: String = String::from("something is not right here bro");
	println!("{}", first_word(&s));
	
	let im_ref: &str = "I am immutable reference";
	println!("{}", *im_ref);
	
}


fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i]; // return slice from starting till space
		}
	}
	
	&s[..] // return whole string since it is one word
	
}









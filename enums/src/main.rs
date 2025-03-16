enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),

	fn route(&self) {
		
	}

}


fn main() {
	let home = IpAddr::V4(12, 12, 12, 12);
	home.route();

}





// much better
#[derive(Debug)] // this is necessary to print Rectangle in println!() trait but for debugging purposes
struct Rectangle {
	width: u32,
	height: u32,
}


fn main() 
{
	// bad
    let width1 = 30;
    let height1 = 50;

    println!(
		"Thre area of the rectangle is {} square pixels",
		area_bad(width1, height1)
    );
    
    // better
    let rect1 = (30, 50);
    println!(
		"Thre area of the rectangle is {} square pixels",
		area_better(rect1)
    );
    
    // much better
	let rect2 = Rectangle {
		width: 30,
		height: 50,
	};
	println!(
		"Thre area of the rectangle is {} square pixels",
		area_much_better(&rect2)
    );
    
    //println!("{}", rect2);  this won't work
    println!("{:?}", rect2);  
    // requires #[derive(Debug)] annotation on struct
    println!("{:#?}", rect2); // when we have large structs this is better
    let rect2 = dbg!(rect2); // this will work but it takes ownership and returns 
    
	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale),
		height: 50,
	};
    
    dbg!(&rect1);

    
}


fn area_much_better(dimensions: &Rectangle) -> u32
{
	(*dimensions).width * (*dimensions).height
}



fn area_better(dimensions: (u32, u32)) -> u32
{
	dimensions.0 * dimensions.1

}

fn area_bad(width: u32, height: u32) -> u32
{
	width * height
}


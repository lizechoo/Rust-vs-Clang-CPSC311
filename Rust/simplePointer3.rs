fn main() {
    // read https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html
    
    #[allow(dead_code)]
    let mut x :i32 = 7;
    #[allow(dead_code)]
    let y :i32 = 9;
	
	println!("[1] x is {}", x);
    println!("[1] y is {}", y);
	
	// introduce scope
	// This avoids dangling pointer which will happen in C after freeing p:
	// Rust frees memory after leaving the scope
	// BUT, is there a downside of this? less freedom?
	{
		let p = &mut x; // borrow occurs here
		println!("[1] p is {:p}", p);  
		println!("[1] *p is {}", *p);
		*p = 8; 
	}
	
	// {
		// let b = &mut x; // borrow occurs here
		// println!("[1] b is {:p}", b);  
		// println!("[1] *b is {}", *b);
		// *b = 3; 
	// }    // uncomment this to see b also updates x to 3
     
    println!("[2] x is {}", x);   // now we can print x, and x is 
				// updated by *p inside of the scope
}
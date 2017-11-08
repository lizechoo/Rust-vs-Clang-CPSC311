fn main() {
    // read https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html
    
    #[allow(dead_code)]
    let mut x :i32 = 7;
    #[allow(dead_code)]
    let y :i32 = 9;
    
    let p = &mut x; // borrow occurs here
	// Note that:
	// let a = &b;
	// let ref a = b;  // are equivalent

    
    // println!("[1] x is {}", x); // this would gives an error b/c in println! 
				// there's a (&mut T) pointing to x,
				//
				// Rust has rules on borrowing, which are only one of the  
				// following can happend at the same time:
				// - one or more references (&T) to a resource
				// - exactly one mutable reference (&mut T)
				//
				// (&T) reference is readable only, so it's save to have more than
				// one occurance. 
				// However, (&mut T) allows pointer that points to x
				// updates value of x; if we have multiple "writable" pointers pointing 
				// the same memory, then the operations are not synchronized b/c at least
				// one pointer will be updating the value. 
				// 
				// This will then introduce us to the concept of scope
				// plz move to simplePointer3.rs
    println!("[1] y is {}", y);
    
	println!("[1] p is {:p}", p);  
    println!("[1] *p is {}", *p);
    
    *p = 8;  
    //println!("[2] x is {}", x);
	println!("[2] p is {:p}", p);
    println!("[2] *p is {}", *p);
}
fn main() {
    //println!("Hello, world!");
    
    #[allow(dead_code)]
    let x :i32 = 7;
    #[allow(dead_code)]
    let y :i32 = 9;
    
    let p = &x; // borrow occurs here

    
    println!("[1] x is {}", x);
    println!("[1] y is {}", y);
    
	println!("[1] p is {:p}", p);  // http://blog.zgtm.de/1 says, address is not 
				//a writable value, so accessing p will recursively 
				//dereference it untill it gets a value.
				// To print the address of p, use {:p} instead of just {}
    println!("[1] *p is {}", *p);
    
	// we cannot do x = 8 and expected *p changes as well, 
	// error: - borrow of `x` occurs at -> let p = &x;
	
	// we cannot do *p = 8 either b/c p is not a mutable borrow
	// error: cannot borrow as mutable
	
	// To make this work, move to simplePointer2
    //*p = 8; 
	
    // println!("[2] x is {}", x);
	// println!("[2] p is {:p}", p);
    // println!("[2] *p is {}", *p);
}
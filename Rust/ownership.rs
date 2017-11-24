fn main() {
    // understanding ownership

    let mut a: i32 = 3;
    println!("a is {}", a);

    let mut d: i32 = 1;

    let b = &mut a;
    *b = 4;
    println!("b is {}", b);

    let mut c = b; // value of b move to c
    		// this happens because b is a type (&mut i32), 
    		// which does not implement "Copy" trait.
    		// What happened is 

    *b = 6; // cannot do this here after move
    //println!("b is {}", b);  // cannot do this either
	println!("c is {}", c);

	*c = 8;

	println!("c is {}", c);

	// what if we change c?
	c = &mut d;
	//println!("b is {}", b); // still can't reference b
	println!("c is {}", c);

	println!("\n\n");

	// if just immutable borrow...
	let x : i32 = 7;
	let p = &x;

	println!("x is {}", x);
	println!("p is {}", p);

	let p2 = p;

	// all works fine b/c it just copy the value to a different location
	// it's not a pointer points to 
	println!("x is {}", x);
	println!("p is {}", p);
	println!("p2 is {}", p2);
}
fn main() {
    // understanding ownership

    let mut a: i32 = 3;
    println!("a is {}", a);

    let b = &mut a;
    *b = 4;
    println!("b is {}", b);

    let c = b; // value of b move to c
    		// this happens because b is a type (&mut i32), 
    		// which does not implement "Copy" trait.
    		// What happened is 

    //*b = 6; // cannot do this here after move

	println!("c is {}", c);
}
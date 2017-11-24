fn main() {
    // understanding ownership

    let mut a: [i32; 3] = [1, 2, 3];
    println!("a is {}, {}, {}", a[0], a[1], a[2]);

    let p = &mut a;
    println!("p is {}, {}, {}", p[0], p[1], p[2]);


    let v = vec![1, 2, 3];
	let mut v2 = v;

	println!("v is {:?}", v);
	println!("v2 is {:?}", v2);
}
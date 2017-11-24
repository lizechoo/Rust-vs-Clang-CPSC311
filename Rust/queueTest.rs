// just a file for simple coding to get start with the final project
use std::collections::VecDeque;

// goal
// implement a queue of paths
// or to be simple, just a queue of numbers
// a queue is like a array but dynamically grows or shrink with push and pop
// we can use VecDeque and its push_back and pop_front to simulate queue

// implement a path of nodes
// to be simple, just a path of numbers (like linklist)
// we can simply use vector (vec!) for the path structure

fn main() {
	// some test with vec!
    let mut v = vec![1, 2, 3];

	let three = v.pop();
   	println!("what just pop? {:?}", three);
   	let val = v.pop();
   	match val {
   		Some(ref p) => println!("what just pop again? {}\n", p),
   		None => println!("just pop a None\n"),
   	}
   	//println!("print a int: {:?}\n", 1);
   	println!("what's v now?: {:?}", v);
   	// notice that vec! pops the last elements...

   	// we can push something to v
   	v.push(5);
   	v.push(6);
   	println!("what's v after push?: {:?}\n", v);

   	

   	// some test with VecDeque
   	let mut testQ = VecDeque::new();
   	testQ.push_back(1);
   	testQ.push_back(2);
   	testQ.push_back(3);
   	testQ.push_back(4);
   	testQ.push_back(5);

   	println!("check testQ: {:?}", testQ);
   	println!("and it's length: {}", testQ.len());

   	// we can read a specific element by
   	println!("last element is: {:?}", testQ.get(testQ.len()-1));
   	// we can simulate queue using pop_front and push_back function of VecDeque

   	let firstE = testQ.pop_front();
   	println!("testQ just pop: {:?}", firstE);
   	println!("now testQ is: {:?}\n", testQ);
   	
   	// we can modify this first element and push it back to testQ
   	let mut e1 :i32 = 0;
   	match firstE {
   		Some(p) => e1 = p, // can be something like this: 
   						// referencing p -> Some(ref p) => e1 = *p,
   		None => println!("Oh no somehow it's a None"),
   	}
   	println!("gets what's inside the Some: {}", e1);
   	e1 += 10;
   	testQ.push_back(e1);
   	println!("pushed a modified e1, now testQ is: {:?}\n", testQ);
}
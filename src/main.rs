// cargo build --target=mips-unknown-linux-gnu
use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    println!("Hello, lovely vuduo!");
	
	let y: f32  = 3.0;
	let x: f32 = 2.0;

	//let y  = 3.0;
	//let x = 7.0;

	let z = x * y;
	if z > 4.0
	{
		println!("bigger than 4");
		//return;
	}
	println!("x={:.1} y={:.1} x/y={:}",x ,y , x / y);


	// Make a vector to hold the children which are spawned.
	let mut children = vec![];

	for i in 0..NTHREADS {
		// Spin up another thread
		children.push(thread::spawn(move || {
			println!("this is thread number {}", i);
		}));
	}

	for child in children {
		// Wait for the thread to finish. Returns a result.
		let _ = child.join();
	}


    println!("you dipshit");
}

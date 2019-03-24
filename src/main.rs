// cargo build --target=mips-unknown-linux-gnu

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

    println!("you dipshit");
}

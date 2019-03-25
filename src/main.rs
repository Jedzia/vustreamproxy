// cargo build --target=mips-unknown-linux-gnu
extern crate rand;

use std::error::Error;
use std::thread;

use rand::Rng;

static NTHREADS: i32 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, lovely vuduo!");

    let y: f32 = 3.0;
    let x: f32 = 2.0;

    //let y  = 3.0;
    //let x = 7.0;

    let z = x * y;
    if z > 4.0 {
        println!("bigger than 4");
        //return;
    }

    println!("x={:.1} y={:.1} x/y={:}", x, y, x / y);

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut z1: f64 = (z * 0.999).into();
            for _j in 0..100000 {
                let y1: f64 = rng.gen(); // generates a float between 0 and 1
                z1 += y1;
                let hit = (y1 * 10000.0).round() / 10000.0;
                if hit == 0.42 {
                    println!("    I[{}] got a 42%, yeah;)", i);
                }

                if y1 > 0.9999 {
                    break;
                }
            }
            println!("this is thread number {} finishing -> {}", i, z1);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    println!("finished.");

    if z < 4.0 {
        // https://stackoverflow.com/questions/51550167/how-to-manually-return-a-result-boxerror
        return Err("Bad request".into());
    }

    Ok(())
}

// cargo build --target=mips-unknown-linux-gnu
//extern crate rand;
#![deny(warnings)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Client, Server};
use std::error::Error;
use std::net::SocketAddr;

//use std::thread;

//use rand::Rng;

//static NTHREADS: i32 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, lovely VU Duo!");

    let y: f32 = 5.0;

    if y < 4.0 {
        // https://stackoverflow.com/questions/51550167/how-to-manually-return-a-result-boxerror
        return Err("Bad request".into());
    }

    // http://192.168.2.43:3000/
    //let addr = ([0, 0, 0, 0], 3000).into();
    //let addr = ([127, 0, 0, 1], 3000).into();
    //let addr = ([192, 168, 2, 43], 3000).into();

    pretty_env_logger::init();

    let in_addr = ([127, 0, 0, 1], 3001).into();
    let out_addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    // google.de 216.58.208.35
    //let out_addr: SocketAddr = ([216, 58, 208, 35], 443).into();

    let client_main = Client::new();

    let out_addr_clone = out_addr.clone();
    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let new_service = move || {
        let client = client_main.clone();
        // This is the `Service` that will handle the connection.
        // `service_fn_ok` is a helper to convert a function that
        // returns a Response into a `Service`.
        service_fn(move |mut req| {
            let uri_string = format!(
                "http://{}/{}",
                out_addr_clone,
                req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("")
            );
            let uri = uri_string.parse().unwrap();

            let in_uri_string = format!("http://{}/{}", in_addr, req.uri());
            let in_remove_string = format!("http://{}//", in_addr);
            println!("req.uri(): {}", in_uri_string);
            let result = in_uri_string.replace(&in_remove_string, "");

            //let result = in_uri_string.split(in_remove_string.unwrap_or("")).take(1).next().unwrap_or("");

            println!("result: {}", result);

            *req.uri_mut() = uri;
            client.request(req)
        })
    };

    let server = Server::bind(&in_addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", in_addr);
    println!("Proxying on http://{}", out_addr);

    rt::run(server);

    println!("finished.");
    Ok(())
}

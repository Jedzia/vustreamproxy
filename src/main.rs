// cargo build --target=mips-unknown-linux-gnu
//extern crate rand;
//#![deny(warnings)]
#![feature(type_ascription)]

extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use futures::{future, Async, Poll};
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Chunk, Method, Request, Response, Server, StatusCode, Uri};
//use hyper::{Client, Server, Method, Body, Response, Request};
use std::error::Error;
use std::net::SocketAddr;

use std::fs::File;
//use std::io;
//use std::io::prelude::*;

use std::io::prelude::*;
use std::process::{Command, Stdio};

//use std::thread;

//use rand::Rng;

//static NTHREADS: i32 = 10;

fn get_in_addr() -> SocketAddr {
    let mut in_addr: SocketAddr = ([192, 168, 3, 43], 3333).into();

    if cfg!(target_os = "windows") {
        in_addr: SocketAddr = ([127, 0, 0, 1], 3333).into();
    }
    return in_addr;
}

fn reduce_forwarded_uri(uri: &Uri) -> String {
    //let in_addr: SocketAddr = get_in_addr();
    let uri_string = uri.path_and_query().map(|x| x.as_str()).unwrap_or("");
    //let uri: String = uri_string.parse().unwrap();

    //let in_uri_string = format!("http://{}/{}", in_addr, req.uri());
    let in_remove_string = "/fwd/";
    debug!("uri_string: {}", uri_string);
    let result = uri_string.replace(&in_remove_string, "");
    debug!("result: {}", result);

    //let result = in_uri_string.split(in_remove_string.unwrap_or("")).take(1).next().unwrap_or("");
    result
}

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
//fn echo(req: Request<Body>, buf: Vec<u8>) -> BoxFut {
fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());
    debug!("method: {}, uri: {}", req.method(), req.uri());

    match req.method() {
        &Method::GET => {
            if req.uri().path().starts_with("/fwd/") {
                *response.body_mut() =
                    Body::from("Lets forward: ".to_owned() + &reduce_forwarded_uri(req.uri()));
            }
        }
        _ => {}
    }

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            //command_name = 'ffmpeg',
            //command_opts = ['-i', 'pipe:0', '-f', 'mp3', '-acodec', 'libvorbis', '-ab', '128k', '-aq', '60', '-f', 'ogg', '-'];
            /*
                        let command_name = "ffmpeg";
                        //let command_opts = ["-i", "pipe:0", "-f", "mp3", "-acodec", "libvorbis", "-ab", "128k", "-aq", "60", "-f", "ogg", "-"];
                        //"D:\Program Files\ffmpeg\bin\ffmpeg" -re -i "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg"
                        // -acodec libmp3lame -ab 128k -aq 60 -f mp3 - > bla.mp3

                        //let media_addr = "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg";
                        let media_addr = "https://upload.wikimedia.org/wikipedia/commons/f/f2/Median_test.ogg";
                        let command_opts = ["-i", media_addr,
                            "-acodec", "libmp3lame", "-ab", "128k", "-aq", "60", "-f", "mp3", "-"];
                        let mut ffmpeg_path = command_name;
                        if cfg!(target_os = "windows") {
                            ffmpeg_path = "D:/Program Files/ffmpeg/bin/ffmpeg.exe";
                        }

                        // Spawn the `wc` command
                        let process = match Command::new(ffmpeg_path)
                            .args(&command_opts)
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
                        {
                            Err(why) => panic!("couldn't spawn {}: {}", command_name, why.description()),
                            Ok(process) => process,
                        };

                        // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
                        let mut buffer: Vec<u8> = Vec::new();
                        match process.stdout.unwrap().read_to_end(&mut buffer) {
                            Err(why) => panic!("couldn't read {} stdout: {}", command_name, why.description()),
                            Ok(_) => println!("buffer size:[{}]", buffer.len()),
                        }
            */

            /*let mapping = || -> Vec(u8)
            {

            }*/

            //let chunks = vec!["hello", " ", "world"];
            //let stream = futures::stream::iter_ok::<_, ::std::io::Error>(chunks);

            /*let mapping = req
            .into_body()
            .map(|chunk| {
                chunk.iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });*/

            let mapping1 = req.into_body().map(|chunk| {
                chunk
                    .iter()
                    .map(|byte| {
                        println!("chunk {}", byte.to_ascii_uppercase());
                        byte.to_ascii_uppercase()
                    })
                    .collect::<Vec<u8>>()
            });

            let data_fuck = vec!["FUCK", " ", "YOU!"];
            let chunk_fuck = Chunk::from("fuck");
            let stream_fuck = futures::stream::iter_ok::<_, ::std::io::Error>(data_fuck);

            //let data2 = vec!["hello", " ", "world"];
            let data2: Vec<u8> = vec![0x55, 0x20, 0x66];
            //let chunk2 = Chunk::from(data2);

            //let conv = |x: Vec<u8>| x.iter();

            let stream2 = futures::stream::iter_ok::<_, ::std::io::Error>(data2);
            //let stream2 = futures::stream::iter_ok::<_, ::std::io::Error>(data2);

            let chunks = load_local_mp3_buffer();
            let c: &[u8] = &chunks; // c: &[u8]
                                    //let chunk = Chunk::from(c);
            let stream = futures::stream::iter_ok::<_, ::std::io::Error>(c);

            // type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
            //let bbb = Box::new(future::ok(Response::new("Fuck YOU")));
            //let xxx: BoxFut = Box::new(future::ok(response));
            //xxx

            let my_stream = MyStream::new(5);
            //let xstream = futures::stream::iter_ok::<_, ::std::io::Error>(my_stream.iter());


            *response.body_mut() = Body::wrap_stream(stream_fuck);
            //*response.body_mut() = Body::empty();
            //*response.set_body(Box::new(stream));
        }

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        //(&Method::GET, Some("/fwd/")) => {
        //    *response.body_mut() = Body::from("Jahahahahaha");
        //}
        // Convert to uppercase before sending back to client.
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req.into_body().map(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });

            *response.body_mut() = Body::wrap_stream(mapping);
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive. So, this returns a different
        // future, waiting on concatenating the full body, so that
        // it can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let reversed = req.into_body().concat2().map(move |chunk| {
                let body = chunk.iter().rev().cloned().collect::<Vec<u8>>();
                *response.body_mut() = Body::from(body);
                response
            });

            return Box::new(reversed);
        }

        // The 404 Not Found route...
        _ => {
            println!("404 not found.");
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}


struct MyStream {
    current: u32,
    max: u32,
}

impl MyStream {
    pub fn new(max: u32) -> MyStream {
        MyStream {
            current: 0,
            max: max,
        }
    }
}

impl Stream for MyStream {
    type Item = u32;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.current {
            ref mut x if *x < self.max => {
                *x = *x + 1;
                Ok(Async::Ready(Some(*x)))
            }
            _ => Ok(Async::Ready(None)),
        }
    }
}

fn load_local_mp3_buffer() -> Vec<u8> {
    //let mut f = File::open("./p.mp3").expect("failed to open mp3 file!");
    let mut filepath = "/media/hdd/jedzia/rust/p.mp3";
    if cfg!(target_os = "windows") {
        filepath = "p.mp3";
    }

    let mut f = File::open(filepath).expect("failed to open mp3 file!");
    //let mut f = File::open(filepath);
    //if let Err(err) = f{
    //    error!("failed to open mp3 file! {}", err);
    //}
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("failed to read mp3 file.");
    buffer
}

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

    //let mut buffer = String::new();
    //f.read_to_string(&mut buffer)?;

    //let in_addr: SocketAddr = ([127, 0, 0, 1], 3333).into();

    let in_addr = get_in_addr();

    /*let out_addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
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

    rt::run(server);*/

    //let mut f = File::open("p.mp3")?;
    //let mut buffer: Vec<u8> = Vec::new();
    //f.read_to_end(&mut buffer)?;
    //let b = buffer.clone();

    let server = Server::bind(&in_addr)
        //.serve(|| service_fn(|req| echo(req, Vec::new())))
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", in_addr);
    hyper::rt::run(server);

    println!("finished.");
    Ok(())
}

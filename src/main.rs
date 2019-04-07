// cargo build --target=mips-unknown-linux-gnu
//extern crate rand;
//#![deny(warnings)]
#![feature(type_ascription)]

#[macro_use]
extern crate log;

extern crate bytes;
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate regex;
//extern crate reqwest;
//extern crate tokio;

#[macro_use]
extern crate lazy_static;

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

use core::borrow::Borrow;
use futures::future::FutureResult;
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

use regex::Regex;
use std::borrow::Cow;

mod chunk;
mod fileio;
mod stream;

//use tokio::codec::{Decoder, FramedRead};
//use tokio::prelude::{AsyncRead};
//use tokio::fs::File;
//use tokio::io;

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

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX : Regex = Regex::new(
            //r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})"
            //r"/^(?:(?:(?:https?|ftp):)?\/\/)(?:\S+(?::\S*)?@)?(?:(?!(?:10|127)(?:\.\d{1,3}){3})(?!(?:169\.254|192\.168)(?:\.\d{1,3}){2})(?!172\.(?:1[6-9]|2\d|3[0-1])(?:\.\d{1,3}){2})(?:[1-9]\d?|1\d\d|2[01]\d|22[0-3])(?:\.(?:1?\d{1,2}|2[0-4]\d|25[0-5])){2}(?:\.(?:[1-9]\d?|1\d\d|2[0-4]\d|25[0-4]))|(?:(?:[a-z0-9\u00a1-\uffff][a-z0-9\u00a1-\uffff_-]{0,62})?[a-z0-9\u00a1-\uffff]\.)+(?:[a-z\u00a1-\uffff]{2,}\.?))(?::\d{2,5})?(?:[/?#]\S*)?$/i"
            //r"(?:(?:(?:https?|ftp):)?\/\/)(?:\S+(?::\S*)?@)?(?:(?!(?:10|127)(?:\.\d{1,3}){3})(?!(?:169\.254|192\.168)(?:\.\d{1,3}){2})(?!172\.(?:1[6-9]|2\d|3[0-1])(?:\.\d{1,3}){2})(?:[1-9]\d?|1\d\d|2[01]\d|22[0-3])(?:\.(?:1?\d{1,2}|2[0-4]\d|25[0-5])){2}(?:\.(?:[1-9]\d?|1\d\d|2[0-4]\d|25[0-4]))|(?:(?:[a-z0-9\u00a1-\uffff][a-z0-9\u00a1-\uffff_-]{0,62})?[a-z0-9\u00a1-\uffff]\.)+(?:[a-z\u00a1-\uffff]{2,}\.?))(?::\d{2,5})?(?:[/?#]\S*)?$/i"
            r"(https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)*.(\.ogg))"
            ).unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "FUCK YA")
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
                let req_uri = reduce_forwarded_uri(req.uri());
                //let forwarded_uri = Uri::from_static(&req_uri);
                *response.body_mut() = Body::from("Lets forward: ".to_owned() + &req_uri);

                /*let body = reqwest::get(req_uri.as_str())//.unwrap();
                //.danger_disable_certs_verification()
                .expect(&format!("cannot get '{}'", &req_uri))
                .text()//.unwrap();
                .expect(&format!("cannot get text for '{}'", &req_uri));*/

                let body = reqwest::Client::builder()
                    .danger_accept_invalid_hostnames(true)
                    .danger_accept_invalid_certs(true)
                    .build()
                    .unwrap()
                    .get("https://www.google.de/")
                    .send()
                    .unwrap()
                    .text()
                    .unwrap();

                println!("body = {}", body);

                /*let re_weburl = Regex::new(
                    r"/^(?:(?:(?:https?|ftp):)?\/\/)(?:\S+(?::\S*)?@)?(?:(?!(?:10|127)(?:\.\d{1,3}){3})(?!(?:169\.254|192\.168)(?:\.\d{1,3}){2})(?!172\.(?:1[6-9]|2\d|3[0-1])(?:\.\d{1,3}){2})(?:[1-9]\d?|1\d\d|2[01]\d|22[0-3])(?:\.(?:1?\d{1,2}|2[0-4]\d|25[0-5])){2}(?:\.(?:[1-9]\d?|1\d\d|2[0-4]\d|25[0-4]))|(?:(?:[a-z0-9\u00a1-\uffff][a-z0-9\u00a1-\uffff_-]{0,62})?[a-z0-9\u00a1-\uffff]\.)+(?:[a-z\u00a1-\uffff]{2,}\.?))(?::\d{2,5})?(?:[/?#]\S*)?$/i"
                );*/

                // check if there is an alternative to the ogg-vorbis stream
                // when true, then prioritize the mp3 over it
                // else create a reference to the mp3 forwarding endpoint

                // SSL Certificates on the host are important. make shure:
                //   ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
                //   ENV SSL_CERT_DIR=/etc/ssl/certs
                // are set.

                let after = reformat_dates(&body);
                //println!("body = {}", after);
                //let chunk = Chunk::from(after);
                //*response.body_mut() = Body::from(after.to_string());
                *response.body_mut() = Body::from(after.to_string());
                *response.body_mut() = Body::from("got regex");
                return Box::new(future::ok(response));
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

                        *response.body_mut() = Body::from(buffer);
                        return Box::new( future::ok(response));
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

            /*            //let data2 = vec!["hello", " ", "world"];
                        let data2: Vec<u8> = vec![0x55, 0x20, 0x66];
                        //let chunk2 = Chunk::from(data2);

                        //let conv = |x: Vec<u8>| x.iter();

                        let stream2 = futures::stream::iter_ok::<_, ::std::io::Error>(data2);
                        //let stream2 = futures::stream::iter_ok::<_, ::std::io::Error>(data2);

                        let chunks = fileio::load_local_mp3_buffer();
                        let c: &[u8] = &chunks; // c: &[u8]
                                                //let chunk = Chunk::from(c);
                        let stream = futures::stream::iter_ok::<_, ::std::io::Error>(c);
                        *response.body_mut() = Body::from(chunks);
                        return Box::new( future::ok(response));
            */

            // type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
            //let bbb = Box::new(future::ok(Response::new("Fuck YOU")));
            //let xxx: BoxFut = Box::new(future::ok(response));
            //xxx

            let my_stream = MyStream::new(5);
            //let xstream = futures::stream::iter_ok::<_, ::std::io::Error>(my_stream.iter());

            //let mut file = &get_local_mp3_path();
            let mut filepath = "/media/hdd/jedzia/rust/p.mp3";
            if cfg!(target_os = "windows") {
                filepath = "p.mp3";
            }

            //let file = File::open(filepath).map(file_response).or_else(|_| status_response(StatusCode::NOT_FOUND));
            //.expect("failed to open file")
            //let file = tokio::fs::File::open(filepath).catch_unwind();
            //let fstream = FramedRead::new(file, ChunkDecoder);

            /*fn decode(buf: Vec<u8>) -> Result<Option<Chunk>, io::Error> {
                let len = buf.len();
                if len > 0 {
                    //Ok(Some(buf.iter().take(32).freeze().into()))
                    Ok(Some(buf.iter().take(32).into()))
                } else {
                    Ok(None)
                }
            }

            let akjsd = decode(chunks);*/

            use bytes::{BigEndian, Buf, BufMut, BytesMut, IntoBuf};

            let bytes = b"\x00\x01hello world";
            let mut bytes_buf = bytes.into_buf();
            let bytes_stream = futures::stream::iter_ok::<_, ::std::io::Error>(bytes);

            //*response.body_mut() = Body::wrap_stream(bytes_stream);
            *response.body_mut() = Body::wrap_stream(stream_fuck);
            //*response.body_mut() = Body::empty();
            //*response.set_body(Box::new(stream));

            // type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
            //let future_result = future::ok(response);

            //            let mut buf = BytesMut::with_capacity(1024);
            //            buf.put(&b"hello world"[..]);

            //let mut response1 = Response::new("Fuck");
            //            let mut response1 = Response::new(Body::from(buf.freeze()));

            //            let future_result: FutureResult<Response<Body>, hyper::Error> = future::ok(response1);

            //return Box::new( future_result);
            //let (method, uri, version, headers, body) = req.deconstruct();
            let myresp =
                chunk::handle_request(Request::new(Body::from("Fuck ya to chunk::handle_request")));
            return Box::new(myresp);

            //let future_result: FutureResult<Response<Body>, hyper::Error> = future::ok(response);
            //return Box::new(future_result);
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

/*struct ChunkDecoder;

impl Decoder for ChunkDecoder {
    type Item = Chunk;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Chunk>, io::Error> {
        let len = buf.len();
        if len > 0 {
            Ok(Some(buf.take().freeze().into()))
        } else {
            Ok(None)
        }
    }
}*/

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

/*fn get_local_mp3_path() -> &str {
    //let mut f = File::open("./p.mp3").expect("failed to open mp3 file!");
    let mut filepath = "/media/hdd/jedzia/rust/p.mp3";
    if cfg!(target_os = "windows") {
        filepath = "p.mp3";
    }
    filepath
}*/

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

    //testOpenSSL();

    //let mut buffer = String::new();
    //f.read_to_string(&mut buffer)?;

    //let in_addr: SocketAddr = ([127, 0, 0, 1], 3333).into();

    /*extern crate openssl_static_sys = "openssl-static-sys";
    if cfg!(target_os = "linux") {
        openssl_static_sys::init_ssl_cert_env_vars();
    }*/

    use std::env;
    use std::fs;
    use std::path::PathBuf;
    println!("Entry");
    let cert_file = env::var_os("SSL_CERT_FILE").map(PathBuf::from);
    println!("  env: cert_file {:?}", cert_file);
    let cert_dir = env::var_os("SSL_CERT_DIR").map(PathBuf::from);
    println!("  env: cert_dir {:?}", cert_dir);

    extern crate openssl_probe;
    let ssl = openssl_probe::init_ssl_cert_env_vars();
    println!("cert {:?}", ssl);

    println!("After openssl_probe");
    let cert_file = env::var_os("SSL_CERT_FILE").map(PathBuf::from);
    println!("  env: cert_file {:?}", cert_file);
    let cert_dir = env::var_os("SSL_CERT_DIR").map(PathBuf::from);
    println!("  env: cert_dir {:?}", cert_dir);

/*    // /media/hdd/jedzia/rust/ca-bundle.trust.crt
    env::set_var("SSL_CERT_DIR", "/etc/ssl/certs");
    //env::set_var("SSL_CERT_FILE", "/etc/ssl/certs/ca-certificates.crt");
    env::set_var("SSL_CERT_FILE", "/media/hdd/jedzia/rust/ca-bundle.trust.crt");

    if cfg!(target_os = "windows") {
        //env::set_var("SSL_CERT_DIR", "/etc/ssl/certs");
        env::set_var(
            "SSL_CERT_FILE",
            r"C:\msys64\etc\pki\ca-trust\extracted\openssl\ca-bundle.trust.crt",
        );
        // set SSL_CERT_FILE=C:\msys64\etc\pki\ca-trust\extracted\openssl\ca-bundle.trust.crt
    }
*/
    println!("After env::set_var");
    let cert_file = env::var_os("SSL_CERT_FILE").map(PathBuf::from);
    println!("  env: cert_file {:?}", cert_file);
    let cert_dir = env::var_os("SSL_CERT_DIR").map(PathBuf::from);
    println!("  env: cert_dir {:?}", cert_dir);


    /*    //let cert_file_path = "/etc/ssl/certs/ca-certificates.crt";
        let cert_file_path = "/media/hdd/jedzia/rust/GIAG2.crt";
        let mut buf = Vec::new();
        File::open(cert_file_path).unwrap().read_to_end(&mut buf).unwrap();
        let cert = reqwest::Certificate::from_der(&buf).unwrap();
        println!("  cert {:?}", cert);
    */

    testOpenSSL();

    println!("===== TestBody =====");
    let body = reqwest::Client::builder()
        //.danger_accept_invalid_hostnames(true)
        //.danger_accept_invalid_certs(true)
        //.add_root_certificate(cert)
        .build()
        .unwrap()
        .get("https://www.google.de/")
        .send()
        .unwrap()
        .text()
        .unwrap();

    //let bla = body.lines().take(3).collect::<String>();
    println!("body = {}", body.lines().take(1).collect::<String>());
    return Ok(());

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


/*//#cfg!(target_os = "windows")
fn testOpenSSL1() {
    extern crate openssl;
    use openssl::rsa::{Padding, Rsa};

    let rsa = Rsa::generate(2048).unwrap();
    let data = b"foobar";
    println!("data {:?}", data);
    let mut buf = vec![0; rsa.size() as usize];
    let encrypted_len = rsa.public_encrypt(data, &mut buf, Padding::PKCS1).unwrap();
    println!("encripted {:?}", buf);
}*/

fn testOpenSSL() {
/*    extern crate openssl;
    println!("===== testOpenSSL =====");

    use openssl::ssl::{SslConnector, SslMethod};
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = connector.connect("google.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!(
        "{}",
        String::from_utf8_lossy(&res)
            .lines()
            .take(3)
            .collect::<String>()
    );*/
}
